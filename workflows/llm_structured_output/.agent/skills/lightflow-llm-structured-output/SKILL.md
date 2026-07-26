---
name: LightFlow LLM Structured Output
description: Use this skill when working with the lightflow.llm_structured_output workflow that turns LLM text into JSON.
version: 0.1.0
---

# LightFlow LLM Structured Output

Use `lightflow.llm_structured_output` to parse JSON text or wrap plain text as `{ "text": ... }`.

## Workflow

- Workflow id: `lightflow.llm_structured_output`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-llm-structured-output-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.llm.structured_output`.
- Input `text`: required text or JSON string.
- Input `schema`: optional JSON schema metadata.
- Outputs `object` and `json`.

## Usage

```bash
lfw run lightflow.llm_structured_output -i text='{"intent":"search"}'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.llm_structured_output/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
