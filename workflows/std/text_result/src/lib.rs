use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow!()
        .name("Text Result")
        .description("Normalize generated text into a final result.")
        .input("text", "text")
        .input_description(
            "text",
            "Generated text to expose through the final result port.",
        )
        .input_required("text", true)
        .input_widget("text", "textarea")
        .output("result", "text")
        .output_description("result", "Final normalized text result.")
        .build()
}
