# lightflow-std

Standard LightFlow workflow collection.

## Workflows

- `lightflow.std`: identity passthrough for JSON values.
- `lightflow.text_prompt`: convert structured input into prompt text.
- `lightflow.text_result`: normalize generated text into a result port.
- `lightflow.text_plan`: small composite example built from std workflows.
- `lightflow.text_to_image`: deterministic preview image generation workflow.
- `lightflow.image.invert`: PNG color inversion workflow.

## Usage

Install the collection into a project or global LightFlow home:

```bash
lfw install --global /path/to/lightflow-std
```

Run a workflow:

```bash
lfw run lightflow.std -i value='{"hello":"world"}'
```

Preview publishing all workflow crates:

```bash
lfw publish --workflows
```
