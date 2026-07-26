---
name: LightFlow Text Regex
description: Use this skill when working with the lightflow.text_regex workflow that matches or replaces text with regular expressions.
version: 0.1.0
---

# LightFlow Text Regex

Use `lightflow.text_regex` to run deterministic regex matching or replacement in a workflow.

## Workflow

- Workflow id: `lightflow.text_regex`
- Runtime ownership: the workflow crate owns `execute()` and exposes it through
  the `lightflow-text-regex-runner` package binary.
- Host engine: `package.command.v1`; the runner is declared in
  `[package.metadata.lightflow]` and is started only by an explicit run.
- Input `text`: required source text; widget `textarea`.
- Input `pattern`: required Rust regex pattern.
- Input `replacement`: optional replacement text. When omitted, output `text` is the original input.
- Outputs: `text`, `matched`, `match_count`, `captures`, `first_match`.

## Usage

```bash
lfw run lightflow.text_regex -i text='cat 42' -i pattern='(\d+)' -i replacement='id:$1'
lfw plan lightflow.text_regex
lfw trace last
lfw replay
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.text_regex/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{"text":"cat 42","pattern":"(\\d+)","replacement":"id:$1"}}'
```
