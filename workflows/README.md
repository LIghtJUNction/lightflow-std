# Workflows

Each top-level directory is one category. Workflow crates live one level below
that category, for example `std/text_plan/src/lib.rs`. Reusable workflows define
`src/lib.rs` and do not define `src/main.rs`. Leaf workflows declare ports and
no nodes. Composite workflows use `.node(..., workflow_id)` to nest other
workflows.

Ports should include Node Schema v1 metadata when a UI or agent needs to render
or validate the node contract: descriptions, required/default flags, numeric
ranges, enum values, widget hints, artifact kinds, and model requirement
bindings. Runtime-backed leaf workflows should declare a capability that exists
in LightFlow's Executor Registry.

`lightflow.std` is a normal workflow crate in this repository, not a backend
built-in. It is reserved for minimal, abstract, reusable building blocks and
must not contain agent behavior, provider integrations, or business templates.

`lightflow.text_plan` depends on and nests `lightflow.std` to verify that the
standard workflow path is exercised by a real local workflow.

Every workflow crate in this repository includes a skill at
`.agent/skills/<skill-name>/SKILL.md`. Update that skill whenever inputs,
outputs, model requirements, runtime behavior, or common commands change.
Validate changed workflows with:

```bash
lfw node test <workflow_id>
```
