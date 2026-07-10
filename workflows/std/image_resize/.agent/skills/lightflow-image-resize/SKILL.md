---
name: LightFlow Image Resize
description: Use this skill when working with the lightflow.image_resize workflow that resizes PNG image artifacts.
version: 0.1.0
---

# LightFlow Image Resize

Use `lightflow.image_resize` to resize a PNG image with nearest-neighbor sampling.

## Workflow

- Workflow id: `lightflow.image_resize`
- Runtime: `lightflow.image.resize`.
- Input `image_path`: required source PNG path; artifact kind `image`; widget `image`.
- Inputs `width` and `height`: required output dimensions; widget `number`.
- Input `output_path`: optional destination PNG path; artifact kind `image`; widget `file_save`.
- Outputs `image` and `image_path`; artifact kind `image`.

## Usage

```bash
lfw run lightflow.image_resize -i image_path=./input.png -i width=512 -i height=512
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.image_resize/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
