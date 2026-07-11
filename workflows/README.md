# Workflows

Each top-level directory is one workflow crate, for example
`text_plan/src/lib.rs`. Reusable workflows define `src/lib.rs` and do not define
`src/main.rs`. Leaf workflows declare ports and no nodes. Composite workflows
use `.node(..., workflow_id)` to nest other workflows.

Ports should include Node Schema v1 metadata when a UI or agent needs to render
or validate the node contract: descriptions, required/default flags, numeric
ranges, enum values, widget hints, artifact kinds, and model requirement
bindings. Runtime-backed leaf workflows should declare a capability that exists
in LightFlow's Executor Registry.

Every node is a separate normal workflow crate that can be published and
consumed independently.

`lightflow.text_plan` composes `lightflow.text_prompt` and
`lightflow.text_result` to exercise the standard workflow path.

Every workflow crate in this repository includes a skill at
`.agent/skills/<skill-name>/SKILL.md`. Update that skill whenever inputs,
outputs, model requirements, runtime behavior, or common commands change.
Validate changed workflows with:

```bash
lfw node test <workflow_id>
```
