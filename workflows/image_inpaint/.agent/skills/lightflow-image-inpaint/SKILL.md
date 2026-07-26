---
name: LightFlow Image Inpaint
description: Use this skill when working with the lightflow.image_inpaint workflow that creates deterministic preview inpaint images.
version: 0.1.0
---

# LightFlow Image Inpaint

Use `lightflow.image_inpaint` for masked image editing preview workflows. The builtin runtime edits source pixels according to mask luminance; a model-backed executor can replace it later without changing the workflow interface.

## Workflow

- Workflow id: `lightflow.image_inpaint`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-image-inpaint-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.image.inpaint` with engine `builtin.preview.inpaint.v1`.
- Inputs: `image_path`, `mask_path`, `prompt`, optional `negative`, optional `seed`, optional `output_path`.
- Outputs: `image`, `image_path`, `prompt`.

## Usage

```bash
lfw run lightflow.image_inpaint -i image_path=input.png -i mask_path=mask.png -i prompt='repair the damaged area'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.image_inpaint/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
