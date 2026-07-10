---
name: LightFlow Control If
description: Use this skill when working with the lightflow.control_if workflow that selects between two JSON values.
version: 0.1.0
---

# LightFlow Control If

Use `lightflow.control_if` to choose between `then_value` and `else_value`.

## Workflow

- Workflow id: `lightflow.control_if`
- Runtime: `lightflow.control.if`.
- Input `condition`: required boolean; widget `toggle`.
- Inputs `then_value` and `else_value`: optional JSON values.
- Outputs `value` and `selected`.

## Usage

```bash
lfw run lightflow.control_if -i condition=true -i then_value='"yes"' -i else_value='"no"'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.control_if/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
