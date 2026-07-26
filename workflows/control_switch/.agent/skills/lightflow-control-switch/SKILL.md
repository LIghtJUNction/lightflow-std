---
name: LightFlow Control Switch
description: Use this skill when working with the lightflow.control_switch workflow that selects JSON values by key.
version: 0.1.0
---

# LightFlow Control Switch

Use `lightflow.control_switch` to select a value from a JSON object by selector key.

## Workflow

- Workflow id: `lightflow.control_switch`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-control-switch-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.control.switch`.
- Input `selector`: required case key.
- Input `cases`: required JSON object.
- Input `default`: optional fallback JSON value.
- Outputs `value` and `selected`.

## Usage

```bash
lfw run lightflow.control_switch -i selector=fast -i cases='{"fast":"draft","final":"polished"}' -i default='"draft"'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.control_switch/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
