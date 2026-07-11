# lightflow-std

Standard LightFlow workflow collection. Each workflow is a normal Rust library
crate under `workflows/<short-name>/` and includes an agent skill under
`.agent/skills/<skill-name>/SKILL.md`.

## Workflows

- `lightflow.text_plan`: small composite example built from std workflows.
- `lightflow.text_prompt`: convert structured input into prompt text.
- `lightflow.text_result`: normalize generated text into a result port.
- `lightflow.text_concat`: join text values.
- `lightflow.text_template`: render `{{path}}` placeholders from JSON.
- `lightflow.text_regex`: match or replace text with Rust regex syntax.
- `lightflow.json_extract`: extract a JSON value by dot path.
- `lightflow.text_to_image`: deterministic preview image generation workflow.
- `lightflow.image_invert`: PNG color inversion workflow.
- `lightflow.image_load`: load PNG metadata as an image artifact.
- `lightflow.image_save`: copy a PNG artifact to a destination path.
- `lightflow.image_resize`: resize a PNG image.
- `lightflow.image_crop`: crop a PNG image.
- `lightflow.image_upscale`: nearest-neighbor integer PNG upscale.
- `lightflow.image_edit`: deterministic preview image-to-image edit.
- `lightflow.image_inpaint`: deterministic preview masked inpaint.
- `lightflow.mask_compose`: combine two PNG masks.
- `lightflow.control_if`: select between then/else values.
- `lightflow.control_switch`: select a case value by selector.
- `lightflow.control_merge`: merge two JSON values.
- `lightflow.control_split`: split an array/object/scalar value.
- `lightflow.model_select`: select a model variant from JSON metadata.
- `lightflow.model_lock_check`: inspect a model entry in `lfw.lock`.
- `lightflow.llm_generate`: deterministic mock LLM generation.
- `lightflow.llm_classify`: deterministic label selection.
- `lightflow.llm_structured_output`: parse or wrap text as JSON.

## Usage

Import the collection into a project or global LightFlow home:

```bash
lfw import --global /path/to/lightflow-std
```

Run a workflow:

```bash
lfw run lightflow.text_prompt -i value='{"hello":"world"}'
```

Preview publishing all workflow crates:

```bash
lfw publish --workflows
```

Run node conformance before publishing changed workflows:

```bash
lfw node test lightflow.text_regex
lfw node test lightflow.mask_compose
lfw node test lightflow.image_inpaint
```
