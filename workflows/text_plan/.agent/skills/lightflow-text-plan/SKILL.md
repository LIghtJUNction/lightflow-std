---
name: LightFlow Text Plan
description: Use this skill when working with the lightflow.text_plan composite example workflow.
version: 0.1.0
---

# LightFlow Text Plan

Use `lightflow.text_plan` as a small example of composing workflow nodes and edges.

## Workflow

- Workflow id: `lightflow.text_plan`
- Input `value`: required JSON value passed into the plan; widget `json`.
- Output `result`: normalized text result.
- Dependencies: `lightflow.text_prompt`, `lightflow.text_result`.

## Usage

```bash
lfw run lightflow.text_plan -i value='"hello"'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text_plan/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
