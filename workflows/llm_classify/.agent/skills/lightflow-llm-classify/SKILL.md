---
name: LightFlow LLM Classify
description: Use this skill when working with the lightflow.llm_classify workflow that classifies text into labels offline.
version: 0.1.0
---

# LightFlow LLM Classify

Use `lightflow.llm_classify` to choose a label from a JSON label array. The builtin implementation is deterministic and offline.

## Workflow

- Workflow id: `lightflow.llm_classify`
- Runtime ownership: this workflow crate owns its public `execute()` entry and exposes it through the `lightflow-llm-classify-runner` package binary.
- Host engine: `package.command.v1`; discovery, help, and plan do not execute package source.
- Input values are validated against their declared JSON types without string coercion.
- Runtime: `lightflow.llm.classify`.
- Input `text`: required text.
- Input `labels`: required JSON array of labels.
- Outputs `label` and `confidence`.

## Usage

```bash
lfw run lightflow.llm_classify -i text='urgent billing issue' -i labels='["billing","support"]'
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.llm_classify/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
