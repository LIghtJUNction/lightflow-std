---
name: LightFlow Text Concat
description: Use this skill when working with the lightflow.text_concat workflow that joins text values for prompts or labels.
version: 0.1.0
---

# LightFlow Text Concat

Use `lightflow.text_concat` to join two text inputs or an array of values.

## Workflow

- Workflow id: `lightflow.text_concat`
- Runtime: `lightflow.text.concat`.
- Inputs `a` and `b`: optional text values; widget `textarea`.
- Input `items`: optional JSON array; widget `json`.
- Input `separator`: optional text separator; default `""`.
- Output `text`: concatenated text.

## Usage

```bash
lfw run lightflow.text_concat -i a=hello -i b=world -i separator=' '
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text_concat/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
