---
name: LightFlow Control Merge
description: Use this skill when working with the lightflow.control_merge workflow that merges two JSON values.
version: 0.1.0
---

# LightFlow Control Merge

Use `lightflow.control_merge` to combine two JSON values.

## Workflow

- Workflow id: `lightflow.control_merge`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-control-merge-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.control.merge`.
- Inputs `a` and `b`: optional JSON values.
- Input `mode`: `first_non_null`, `object`, or `array`; default `first_non_null`.
- Outputs `value` and `selected`.

## Usage

```bash
lfw run lightflow.control_merge -i a='{"prompt":"cat"}' -i b='{"seed":1}' -i mode=object
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.control_merge/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
