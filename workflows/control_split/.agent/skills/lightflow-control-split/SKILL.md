---
name: LightFlow Control Split
description: Use this skill when working with the lightflow.control_split workflow that splits JSON arrays or objects.
version: 0.1.0
---

# LightFlow Control Split

Use `lightflow.control_split` to turn a JSON array or object into `first`, `rest`, and `items` outputs.

## Workflow

- Workflow id: `lightflow.control_split`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-control-split-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.control.split`.
- Input `value`: required JSON value.
- Outputs `first`, `rest`, and `items`.

## Usage

```bash
lfw run lightflow.control_split -i value='["prompt","negative","seed"]'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.control_split/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
