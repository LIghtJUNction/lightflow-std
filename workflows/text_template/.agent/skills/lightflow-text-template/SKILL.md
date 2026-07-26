---
name: LightFlow Text Template
description: Use this skill when working with the lightflow.text_template workflow that renders prompt templates from JSON variables.
version: 0.1.0
---

# LightFlow Text Template

Use `lightflow.text_template` to render prompt text with `{{key}}` placeholders. Nested JSON values can be addressed with dot paths such as `{{user.name}}`.

## Workflow

- Workflow id: `lightflow.text_template`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-text-template-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.text.template`.
- Input `template`: required template text; widget `textarea`.
- Input `vars`: optional JSON object; default `{}`; widget `json`.
- Output `text`: rendered text.

## Usage

```bash
lfw run lightflow.text_template -i template='Describe {{topic}}' -i vars='{"topic":"a quiet lake"}'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text_template/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
