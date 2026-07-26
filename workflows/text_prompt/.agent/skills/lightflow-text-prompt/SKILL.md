---
name: LightFlow Text Prompt
description: Use this skill when working with the lightflow.text_prompt workflow that turns structured input into prompt text.
version: 0.1.0
---

# LightFlow Text Prompt

Use `lightflow.text_prompt` to normalize structured input into a text prompt.

## Workflow

- Workflow id: `lightflow.text_prompt`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-text-prompt-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Cargo package `lightflow-text-prompt` supplies this ID and its Cargo package version through `workflow!()`.
- Input `value`: required JSON source value; widget `json`.
- Output `prompt`: text prompt generated from the source value.

## Usage

```bash
lfw run lightflow.text_prompt -i value='"repair the masked area naturally"'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text_prompt/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
