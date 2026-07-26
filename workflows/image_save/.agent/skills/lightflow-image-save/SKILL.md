---
name: LightFlow Image Save
description: Use this skill when working with the lightflow.image_save workflow that copies PNG images to chosen output paths.
version: 0.1.0
---

# LightFlow Image Save

Use `lightflow.image_save` to copy a PNG image to a chosen output path.

## Workflow

- Workflow id: `lightflow.image_save`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-image-save-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.image.save`.
- Input `image_path`: required source PNG path; artifact kind `image`; widget `image`.
- Input `output_path`: optional destination PNG path; artifact kind `image`; widget `file_save`.
- Outputs `image` and `image_path`; artifact kind `image`.

## Usage

```bash
lfw run lightflow.image_save -i image_path=./input.png -i output_path=./out.png
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.image_save/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
