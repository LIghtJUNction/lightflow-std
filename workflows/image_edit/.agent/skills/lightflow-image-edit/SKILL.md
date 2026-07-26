---
name: LightFlow Image Edit
description: Use this skill when working with the lightflow.image_edit workflow that creates deterministic preview image edits.
version: 0.1.0
---

# LightFlow Image Edit

Use `lightflow.image_edit` for image-to-image preview workflows. The builtin runtime creates deterministic preview pixels from the source image and prompt; a model-backed executor can replace it later without changing the workflow interface.

## Workflow

- Workflow id: `lightflow.image_edit`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-image-edit-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.image.edit` with engine `builtin.preview.edit.v1`.
- Inputs: `image_path`, `prompt`, optional `negative`, optional `seed`, optional `output_path`.
- Outputs: `image`, `image_path`, `prompt`.

## Usage

```bash
lfw run lightflow.image_edit -i image_path=input.png -i prompt='make the lighting warmer'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.image_edit/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
