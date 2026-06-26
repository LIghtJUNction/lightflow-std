---
name: LightFlow Text Result
description: Use this skill when working with the lightflow.text_result workflow that normalizes generated text.
version: 0.1.0
---

# LightFlow Text Result

Use `lightflow.text_result` to pass generated text into a final result port.

## Workflow

- Workflow id: `lightflow.text_result`
- Input `text`: required generated text; widget `textarea`.
- Output `result`: final normalized text result.

## Usage

```bash
lfw run lightflow.text_result -i text='"done"'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text_result/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
