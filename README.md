# lightflow-std

Standard LightFlow workflow collection. Each workflow is a normal Rust library
crate under `workflows/std/<short-name>/` and includes an agent skill under
`.agent/skills/<skill-name>/SKILL.md`.

## Workflows

- `lightflow.std`: identity passthrough for JSON values.
- `lightflow.text_plan`: small composite example built from std workflows.
- `lightflow.text_prompt`: convert structured input into prompt text.
- `lightflow.text_result`: normalize generated text into a result port.
- `lightflow.text.concat`: join text values.
- `lightflow.text.template`: render `{{path}}` placeholders from JSON.
- `lightflow.text.regex`: match or replace text with Rust regex syntax.
- `lightflow.json.extract`: extract a JSON value by dot path.
- `lightflow.text_to_image`: deterministic preview image generation workflow.
- `lightflow.image.invert`: PNG color inversion workflow.
- `lightflow.image.load`: load PNG metadata as an image artifact.
- `lightflow.image.save`: copy a PNG artifact to a destination path.
- `lightflow.image.resize`: resize a PNG image.
- `lightflow.image.crop`: crop a PNG image.
- `lightflow.image.upscale`: nearest-neighbor integer PNG upscale.
- `lightflow.image.edit`: deterministic preview image-to-image edit.
- `lightflow.image.inpaint`: deterministic preview masked inpaint.
- `lightflow.mask.compose`: combine two PNG masks.
- `lightflow.control.if`: select between then/else values.
- `lightflow.control.switch`: select a case value by selector.
- `lightflow.control.merge`: merge two JSON values.
- `lightflow.control.split`: split an array/object/scalar value.
- `lightflow.model.select`: select a model variant from JSON metadata.
- `lightflow.model.lock_check`: inspect a model entry in `lfw.lock`.
- `lightflow.llm.generate`: deterministic mock LLM generation.
- `lightflow.llm.classify`: deterministic label selection.
- `lightflow.llm.structured_output`: parse or wrap text as JSON.

## Usage

Import the collection into a project or global LightFlow home:

```bash
lfw import --global /path/to/lightflow-std
```

Run a workflow:

```bash
lfw run lightflow.std -i value='{"hello":"world"}'
```

Preview publishing all workflow crates:

```bash
lfw publish --workflows
```

Run node conformance before publishing changed workflows:

```bash
lfw node test lightflow.text.regex
lfw node test lightflow.mask.compose
lfw node test lightflow.image.inpaint
```
