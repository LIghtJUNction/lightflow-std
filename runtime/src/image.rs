use crate::RuntimeError;
use crate::input::{optional_string, optional_u32, optional_u64, required_string, required_u32};
use crate::text::Output;
use lightflow::serde_json::{Map, Value, json};
use lightflow::workflow::WorkflowArtifact;
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::{Component, Path, PathBuf};

pub(crate) fn execute(
    workflow_id: &str,
    inputs: &Map<String, Value>,
) -> Result<Output, RuntimeError> {
    match workflow_id {
        "lightflow.text_to_image" => preview_generate(inputs),
        "lightflow.image_edit" => preview_edit(inputs, false),
        "lightflow.image_inpaint" => preview_edit(inputs, true),
        "lightflow.mask_compose" => mask_compose(inputs),
        _ => image_file(workflow_id, inputs),
    }
}

fn image_file(workflow_id: &str, inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let root = project_root()?;
    let input_relative = required_string(inputs, "image_path")?;
    let input_path = resolve_input(&root, input_relative)?;
    let image = read_png(&input_path)?;
    let tag = workflow_id.rsplit('.').next().unwrap_or("image");

    if workflow_id == "lightflow.image_load" {
        return image_output(input_relative, image.width, image.height, tag);
    }

    let output_relative = output_path(inputs, tag)?;
    let output_path = resolve_output(&root, &output_relative)?;
    let transformed = match workflow_id {
        "lightflow.image_save" => image,
        "lightflow.image_resize" => {
            let width = required_u32(inputs, "width")?;
            let height = required_u32(inputs, "height")?;
            require_nonzero(width, "width")?;
            require_nonzero(height, "height")?;
            resize(&image, width, height)
        }
        "lightflow.image_crop" => {
            let x = optional_u32(inputs, "x")?.unwrap_or(0);
            let y = optional_u32(inputs, "y")?.unwrap_or(0);
            let width = required_u32(inputs, "width")?;
            let height = required_u32(inputs, "height")?;
            crop(&image, x, y, width, height)?
        }
        "lightflow.image_upscale" => {
            let scale = optional_u32(inputs, "scale")?.unwrap_or(2);
            if !(1..=16).contains(&scale) {
                return Err(RuntimeError::new("input `scale` must be from 1 through 16"));
            }
            let width = image
                .width
                .checked_mul(scale)
                .ok_or_else(|| RuntimeError::new("upscaled width overflows u32"))?;
            let height = image
                .height
                .checked_mul(scale)
                .ok_or_else(|| RuntimeError::new("upscaled height overflows u32"))?;
            resize(&image, width, height)
        }
        "lightflow.image_invert" => invert(image),
        other => {
            return Err(RuntimeError::new(format!(
                "unsupported image workflow: {other}"
            )));
        }
    };
    write_png(&output_path, &transformed)?;
    image_output(&output_relative, transformed.width, transformed.height, tag)
}

fn preview_generate(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let root = project_root()?;
    let prompt = required_string(inputs, "prompt")?;
    optional_string(inputs, "negative")?;
    optional_string(inputs, "model")?;
    let width = optional_u32(inputs, "width")?.unwrap_or(512);
    let height = optional_u32(inputs, "height")?.unwrap_or(512);
    if !(64..=2048).contains(&width) || !(64..=2048).contains(&height) {
        return Err(RuntimeError::new(
            "preview width and height must be from 64 through 2048",
        ));
    }
    let seed = optional_u64(inputs, "seed")?.unwrap_or_else(|| stable_seed(prompt));
    let output_relative = output_path(inputs, "preview")?;
    let output = resolve_output(&root, &output_relative)?;
    let image = preview_image(width, height, seed, prompt);
    write_png(&output, &image)?;
    image_output(&output_relative, width, height, "preview")
}

fn preview_edit(inputs: &Map<String, Value>, inpaint: bool) -> Result<Output, RuntimeError> {
    let root = project_root()?;
    let input_path = resolve_input(&root, required_string(inputs, "image_path")?)?;
    let prompt = required_string(inputs, "prompt")?;
    optional_string(inputs, "negative")?;
    let seed = optional_u64(inputs, "seed")?.unwrap_or_else(|| stable_seed(prompt));
    let image = read_png(&input_path)?;
    let mask = if inpaint {
        let path = resolve_input(&root, required_string(inputs, "mask_path")?)?;
        let mask = read_png(&path)?;
        Some(
            if mask.width == image.width && mask.height == image.height {
                mask
            } else {
                resize(&mask, image.width, image.height)
            },
        )
    } else {
        None
    };
    let edited = edit_image(&image, seed, prompt, mask.as_ref());
    let tag = if inpaint { "inpainted" } else { "edited" };
    let output_relative = output_path(inputs, tag)?;
    let output = resolve_output(&root, &output_relative)?;
    write_png(&output, &edited)?;
    let (mut outputs, artifacts) =
        image_output(&output_relative, edited.width, edited.height, tag)?;
    outputs.insert("prompt".to_owned(), Value::String(prompt.to_owned()));
    Ok((outputs, artifacts))
}

fn mask_compose(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let root = project_root()?;
    let a = read_png(&resolve_input(
        &root,
        required_string(inputs, "mask_a_path")?,
    )?)?;
    let b = read_png(&resolve_input(
        &root,
        required_string(inputs, "mask_b_path")?,
    )?)?;
    let b = if a.width == b.width && a.height == b.height {
        b
    } else {
        resize(&b, a.width, a.height)
    };
    let mode = optional_string(inputs, "mode")?.unwrap_or("max");
    let composed = compose_masks(&a, &b, mode)?;
    let output_relative = output_path(inputs, "mask-composed")?;
    let output = resolve_output(&root, &output_relative)?;
    write_png(&output, &composed)?;
    let artifact = artifact(&output_relative, "mask", composed.width, composed.height);
    Ok((
        Map::from_iter([
            ("mask".to_owned(), artifact_value(&artifact)?),
            ("mask_path".to_owned(), Value::String(output_relative)),
            ("mode".to_owned(), Value::String(mode.to_owned())),
        ]),
        vec![artifact],
    ))
}

fn image_output(path: &str, width: u32, height: u32, tag: &str) -> Result<Output, RuntimeError> {
    let artifact = artifact(path, tag, width, height);
    Ok((
        Map::from_iter([
            ("image".to_owned(), artifact_value(&artifact)?),
            ("image_path".to_owned(), Value::String(path.to_owned())),
        ]),
        vec![artifact],
    ))
}

fn artifact(path: &str, tag: &str, width: u32, height: u32) -> WorkflowArtifact {
    WorkflowArtifact {
        id: format!("lightflow-std-{tag}"),
        kind: if tag.contains("mask") {
            "mask"
        } else {
            "image"
        }
        .to_owned(),
        path: path.to_owned(),
        mime_type: "image/png".to_owned(),
        metadata: Map::from_iter([
            ("height".to_owned(), json!(height)),
            ("width".to_owned(), json!(width)),
        ]),
    }
}

fn artifact_value(artifact: &WorkflowArtifact) -> Result<Value, RuntimeError> {
    lightflow::serde_json::to_value(artifact)
        .map_err(|error| RuntimeError::new(format!("serialize artifact: {error}")))
}

fn project_root() -> Result<PathBuf, RuntimeError> {
    Ok(std::env::current_dir()?.canonicalize()?)
}

fn relative_path(value: &str) -> Result<PathBuf, RuntimeError> {
    let path = Path::new(value);
    if value.trim().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(RuntimeError::new(format!(
            "path must be project-relative and cannot contain `..`: {value}"
        )));
    }
    Ok(path.to_owned())
}

fn resolve_input(root: &Path, value: &str) -> Result<PathBuf, RuntimeError> {
    let path = root.join(relative_path(value)?);
    let canonical = path
        .canonicalize()
        .map_err(|error| RuntimeError::new(format!("cannot open `{value}`: {error}")))?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(RuntimeError::new(format!(
            "input path escapes project root or is not a file: {value}"
        )));
    }
    Ok(canonical)
}

fn resolve_output(root: &Path, value: &str) -> Result<PathBuf, RuntimeError> {
    let path = root.join(relative_path(value)?);
    let parent = path
        .parent()
        .ok_or_else(|| RuntimeError::new("output path has no parent"))?;
    fs::create_dir_all(parent)?;
    let canonical_parent = parent.canonicalize()?;
    if !canonical_parent.starts_with(root) {
        return Err(RuntimeError::new(format!(
            "output path escapes project root: {value}"
        )));
    }
    Ok(path)
}

fn output_path(inputs: &Map<String, Value>, tag: &str) -> Result<String, RuntimeError> {
    Ok(optional_string(inputs, "output_path")?
        .map(str::to_owned)
        .unwrap_or_else(|| format!(".lightflow/artifacts/{tag}.png")))
}

#[derive(Clone)]
struct PngImage {
    width: u32,
    height: u32,
    color_type: png::ColorType,
    channels: usize,
    data: Vec<u8>,
}

fn read_png(path: &Path) -> Result<PngImage, RuntimeError> {
    let decoder = png::Decoder::new(BufReader::new(fs::File::open(path)?));
    let mut reader = decoder
        .read_info()
        .map_err(|error| RuntimeError::new(format!("decode PNG: {error}")))?;
    let size = reader
        .output_buffer_size()
        .ok_or_else(|| RuntimeError::new("PNG is too large to decode"))?;
    let mut data = vec![0; size];
    let info = reader
        .next_frame(&mut data)
        .map_err(|error| RuntimeError::new(format!("decode PNG frame: {error}")))?;
    if info.bit_depth != png::BitDepth::Eight {
        return Err(RuntimeError::new("only 8-bit PNG images are supported"));
    }
    let channels = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Indexed => {
            return Err(RuntimeError::new("indexed PNG images are not supported"));
        }
    };
    data.truncate(info.buffer_size());
    Ok(PngImage {
        width: info.width,
        height: info.height,
        color_type: info.color_type,
        channels,
        data,
    })
}

fn write_png(path: &Path, image: &PngImage) -> Result<(), RuntimeError> {
    let writer = BufWriter::new(fs::File::create(path)?);
    let mut encoder = png::Encoder::new(writer, image.width, image.height);
    encoder.set_color(image.color_type);
    encoder.set_depth(png::BitDepth::Eight);
    let mut output = encoder
        .write_header()
        .map_err(|error| RuntimeError::new(format!("encode PNG header: {error}")))?;
    output
        .write_image_data(&image.data)
        .map_err(|error| RuntimeError::new(format!("encode PNG data: {error}")))
}

fn resize(image: &PngImage, width: u32, height: u32) -> PngImage {
    let mut data = vec![0; width as usize * height as usize * image.channels];
    for y in 0..height {
        let src_y = (u64::from(y) * u64::from(image.height) / u64::from(height)) as u32;
        for x in 0..width {
            let src_x = (u64::from(x) * u64::from(image.width) / u64::from(width)) as u32;
            let src = offset(src_x, src_y, image.width, image.channels);
            let dst = offset(x, y, width, image.channels);
            data[dst..dst + image.channels].copy_from_slice(&image.data[src..src + image.channels]);
        }
    }
    PngImage {
        width,
        height,
        color_type: image.color_type,
        channels: image.channels,
        data,
    }
}

fn crop(
    image: &PngImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<PngImage, RuntimeError> {
    require_nonzero(width, "width")?;
    require_nonzero(height, "height")?;
    if x >= image.width || y >= image.height {
        return Err(RuntimeError::new(
            "crop rectangle must intersect the source image",
        ));
    }
    let width = width.min(image.width - x);
    let height = height.min(image.height - y);
    let mut data = vec![0; width as usize * height as usize * image.channels];
    for row in 0..height {
        let src = offset(x, y + row, image.width, image.channels);
        let dst = offset(0, row, width, image.channels);
        let len = width as usize * image.channels;
        data[dst..dst + len].copy_from_slice(&image.data[src..src + len]);
    }
    Ok(PngImage {
        width,
        height,
        color_type: image.color_type,
        channels: image.channels,
        data,
    })
}

fn invert(mut image: PngImage) -> PngImage {
    let colors = if image.channels == 4 || image.channels == 2 {
        image.channels - 1
    } else {
        image.channels
    };
    for pixel in image.data.chunks_exact_mut(image.channels) {
        for channel in &mut pixel[..colors] {
            *channel = 255 - *channel;
        }
    }
    image
}

fn preview_image(width: u32, height: u32, seed: u64, prompt: &str) -> PngImage {
    let mut data = Vec::with_capacity(width as usize * height as usize * 3);
    let mix = stable_seed(prompt);
    for y in 0..height {
        for x in 0..width {
            let base = seed ^ mix ^ (u64::from(x) << 32) ^ u64::from(y);
            data.push(((x * 255 / width) as u8) ^ base as u8);
            data.push(((y * 255 / height) as u8) ^ (base >> 8) as u8);
            data.push((((x + y) * 127 / (width + height)) as u8) ^ (base >> 16) as u8);
        }
    }
    PngImage {
        width,
        height,
        color_type: png::ColorType::Rgb,
        channels: 3,
        data,
    }
}

fn edit_image(image: &PngImage, seed: u64, prompt: &str, mask: Option<&PngImage>) -> PngImage {
    let mut edited = image.clone();
    let colors = image.channels.min(3);
    let mix = stable_seed(prompt);
    for y in 0..image.height {
        for x in 0..image.width {
            let at = offset(x, y, image.width, image.channels);
            let strength = mask.map(|mask| luminance(mask, x, y)).unwrap_or(255);
            for channel in 0..colors {
                let current = u16::from(edited.data[at + channel]);
                let generated =
                    ((seed ^ mix ^ (u64::from(x) << 32) ^ u64::from(y)) >> (channel * 8)) as u8;
                let blend = (u16::from(generated) * u16::from(strength)
                    + current * (255 - u16::from(strength)))
                    / 255;
                edited.data[at + channel] = ((current * 3 + blend) / 4) as u8;
            }
        }
    }
    edited
}

fn compose_masks(a: &PngImage, b: &PngImage, mode: &str) -> Result<PngImage, RuntimeError> {
    let mut data = vec![0; a.width as usize * a.height as usize];
    for y in 0..a.height {
        for x in 0..a.width {
            let a_value = luminance(a, x, y);
            let b_value = luminance(b, x, y);
            data[y as usize * a.width as usize + x as usize] = match mode {
                "add" => a_value.saturating_add(b_value),
                "multiply" | "intersect" => (u16::from(a_value) * u16::from(b_value) / 255) as u8,
                "min" => a_value.min(b_value),
                "subtract" => a_value.saturating_sub(b_value),
                "max" | "union" => a_value.max(b_value),
                other => return Err(RuntimeError::new(format!("unsupported mask mode: {other}"))),
            };
        }
    }
    Ok(PngImage {
        width: a.width,
        height: a.height,
        color_type: png::ColorType::Grayscale,
        channels: 1,
        data,
    })
}

fn luminance(image: &PngImage, x: u32, y: u32) -> u8 {
    let at = offset(x, y, image.width, image.channels);
    if image.channels >= 3 {
        let r = u16::from(image.data[at]);
        let g = u16::from(image.data[at + 1]);
        let b = u16::from(image.data[at + 2]);
        ((r * 77 + g * 150 + b * 29) / 256) as u8
    } else {
        image.data[at]
    }
}

fn offset(x: u32, y: u32, width: u32, channels: usize) -> usize {
    (y as usize * width as usize + x as usize) * channels
}

fn require_nonzero(value: u32, name: &str) -> Result<(), RuntimeError> {
    if value == 0 {
        Err(RuntimeError::new(format!(
            "input `{name}` must be greater than zero"
        )))
    } else {
        Ok(())
    }
}

fn stable_seed(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}
