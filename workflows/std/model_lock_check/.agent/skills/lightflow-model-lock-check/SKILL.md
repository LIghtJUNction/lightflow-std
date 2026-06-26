---
name: LightFlow Model Lock Check
description: Use this skill when working with the lightflow.model.lock_check workflow that checks lfw.lock model entries.
version: 0.1.0
---

# LightFlow Model Lock Check

Use `lightflow.model.lock_check` to check whether `lfw.lock` contains a model entry for `<workflow_id>::<requirement_id>`.

## Workflow

- Workflow id: `lightflow.model.lock_check`
- Runtime: `lightflow.model.lock.check`.
- Inputs `workflow_id` and `requirement_id`: required text values.
- Outputs `locked`, `exists`, `path`, and `entry`.

## Usage

```bash
lfw run lightflow.model.lock_check -i workflow_id=lightflow.text_to_image -i requirement_id=image_model
```

## API Usage

Start `lfw serve`, then call the workflow through the shared HTTP run contract. Adjust `inputs` to match the workflow contract above.

```bash
curl -sS -X POST http://127.0.0.1:5174/workflows/lightflow.model.lock_check/run \
  -H 'content-type: application/json' \
  -d '{"inputs":{}}'
```
