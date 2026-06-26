---
name: LightFlow Text Template
description: Use this skill when working with the lightflow.text.template workflow that renders prompt templates from JSON variables.
version: 0.1.0
---

# LightFlow Text Template

Use `lightflow.text.template` to render prompt text with `{{key}}` placeholders. Nested JSON values can be addressed with dot paths such as `{{user.name}}`.

## Workflow

- Workflow id: `lightflow.text.template`
- Runtime: `lightflow.text.template`.
- Input `template`: required template text; widget `textarea`.
- Input `vars`: optional JSON object; default `{}`; widget `json`.
- Output `text`: rendered text.

## Usage

```bash
lfw run lightflow.text.template -i template='Describe {{topic}}' -i vars='{"topic":"a quiet lake"}'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text.template/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
