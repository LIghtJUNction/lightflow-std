---
name: LightFlow Mask Compose
description: Use this skill when working with the lightflow.mask.compose workflow that combines two PNG masks.
version: 0.1.0
---

# LightFlow Mask Compose

Use `lightflow.mask.compose` to combine two PNG masks into a grayscale mask artifact for image editing or inpainting workflows.

## Workflow

- Workflow id: `lightflow.mask.compose`
- Runtime: `lightflow.mask.compose`.
- Inputs: `mask_a_path`, `mask_b_path`, optional `mode`, optional `output_path`.
- Modes: `max`, `add`, `multiply`, `min`, `subtract`, `union`, `intersect`.
- Outputs: `mask`, `mask_path`, `mode`.

## Usage

```bash
lfw run lightflow.mask.compose -i mask_a_path=mask-a.png -i mask_b_path=mask-b.png -i mode=max
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.mask.compose/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
