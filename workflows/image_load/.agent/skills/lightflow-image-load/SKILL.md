---
name: LightFlow Image Load
description: Use this skill when working with the lightflow.image_load workflow that imports PNG paths as LightFlow image artifacts.
version: 0.1.0
---

# LightFlow Image Load

Use `lightflow.image_load` to validate a PNG file and expose it as an image artifact handle.

## Workflow

- Workflow id: `lightflow.image_load`
- Runtime: `lightflow.image.load`.
- Input `image_path`: required source PNG path; artifact kind `image`; widget `image`.
- Outputs `image` and `image_path`; artifact kind `image`.

## Usage

```bash
lfw run lightflow.image_load -i image_path=./input.png
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.image_load/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
