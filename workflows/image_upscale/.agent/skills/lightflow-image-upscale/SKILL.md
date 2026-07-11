---
name: LightFlow Image Upscale
description: Use this skill when working with the lightflow.image_upscale workflow that enlarges PNG image artifacts.
version: 0.1.0
---

# LightFlow Image Upscale

Use `lightflow.image_upscale` to enlarge a PNG image by an integer scale factor.

## Workflow

- Workflow id: `lightflow.image_upscale`
- Runtime: `lightflow.image.upscale`.
- Input `image_path`: required source PNG path; artifact kind `image`; widget `image`.
- Input `scale`: optional integer; default `2`.
- Outputs `image` and `image_path`; artifact kind `image`.

## Usage

```bash
lfw run lightflow.image_upscale -i image_path=./input.png -i scale=2
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.image_upscale/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
