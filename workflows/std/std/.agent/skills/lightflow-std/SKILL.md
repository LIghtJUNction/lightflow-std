---
name: LightFlow Std
description: Use this skill when working with the lightflow.std identity workflow or composing simple JSON passthrough stages.
version: 0.1.0
---

# LightFlow Std

Use `lightflow.std` as a domain-neutral identity workflow.

## Workflow

- Workflow id: `lightflow.std`.
- Input `value`: required JSON value; widget `json`.
- Output `value`: the same JSON value.

## Usage

```bash
lfw run lightflow.std -i value='{"hello":"world"}'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.std/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
