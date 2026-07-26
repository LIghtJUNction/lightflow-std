---
name: LightFlow Mask Compose
description: Use this skill when working with the lightflow.mask_compose workflow that combines two PNG masks.
version: 0.1.0
---

# LightFlow Mask Compose

Use `lightflow.mask_compose` to combine two PNG masks into a grayscale mask artifact for image editing or inpainting workflows.

## Workflow

- Workflow id: `lightflow.mask_compose`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-mask-compose-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.mask.compose`.
- Inputs: `mask_a_path`, `mask_b_path`, optional `mode`, optional `output_path`.
- Modes: `max`, `add`, `multiply`, `min`, `subtract`, `union`, `intersect`.
- Outputs: `mask`, `mask_path`, `mode`.

## Usage

```bash
lfw run lightflow.mask_compose -i mask_a_path=mask-a.png -i mask_b_path=mask-b.png -i mode=max
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.mask_compose/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
