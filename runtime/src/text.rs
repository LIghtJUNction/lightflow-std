use crate::RuntimeError;
use crate::input::{
    optional_array, optional_object, optional_string, required, required_array, required_string,
};
use lightflow::serde_json::{Map, Value, json};

pub(crate) type Output = (
    Map<String, Value>,
    Vec<lightflow::workflow::WorkflowArtifact>,
);

pub(crate) fn prompt(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let value = required(inputs, "value")?;
    let text = json_text(value);
    Ok((
        Map::from_iter([("prompt".to_owned(), Value::String(text))]),
        Vec::new(),
    ))
}

pub(crate) fn result(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let text = required_string(inputs, "text")?;
    Ok((
        Map::from_iter([("result".to_owned(), Value::String(text.to_owned()))]),
        Vec::new(),
    ))
}

pub(crate) fn concat(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let separator = optional_string(inputs, "separator")?.unwrap_or_default();
    let items = if let Some(items) = optional_array(inputs, "items")? {
        items
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value.as_str().ok_or_else(|| {
                    RuntimeError::new(format!("input `items[{index}]` must be a JSON string"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        ["a", "b"]
            .into_iter()
            .map(|name| optional_string(inputs, name))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect()
    };
    Ok((
        Map::from_iter([("text".to_owned(), Value::String(items.join(separator)))]),
        Vec::new(),
    ))
}

pub(crate) fn template(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let template = required_string(inputs, "template")?;
    let vars = optional_object(inputs, "vars")?;
    let rendered = render_template(template, vars);
    Ok((
        Map::from_iter([("text".to_owned(), Value::String(rendered))]),
        Vec::new(),
    ))
}

pub(crate) fn json_extract(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let source = required(inputs, "value")?;
    let path = required_string(inputs, "path")?;
    let value = lookup_json_path(source, path)
        .cloned()
        .unwrap_or(Value::Null);
    Ok((
        Map::from_iter([
            ("found".to_owned(), Value::Bool(!value.is_null())),
            ("text".to_owned(), Value::String(json_text(&value))),
            ("value".to_owned(), value),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn llm_generate(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let prompt = required_string(inputs, "prompt")?;
    let model = optional_string(inputs, "model")?.unwrap_or("mock");
    let text = format!("mock:{model}:{prompt}");
    Ok((
        Map::from_iter([
            ("response".to_owned(), Value::String(text.clone())),
            ("text".to_owned(), Value::String(text)),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn llm_classify(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let text = required_string(inputs, "text")?;
    let labels = required_array(inputs, "labels")?
        .iter()
        .enumerate()
        .map(|(index, value)| {
            value.as_str().ok_or_else(|| {
                RuntimeError::new(format!("input `labels[{index}]` must be a JSON string"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let lower = text.to_ascii_lowercase();
    let label = labels
        .iter()
        .find(|label| lower.contains(&label.to_ascii_lowercase()))
        .or_else(|| labels.first())
        .copied()
        .unwrap_or_default();
    Ok((
        Map::from_iter([
            (
                "confidence".to_owned(),
                json!(if label.is_empty() { 0.0 } else { 1.0 }),
            ),
            ("label".to_owned(), Value::String(label.to_owned())),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn llm_structured_output(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let text = required_string(inputs, "text")?;
    if inputs.contains_key("schema") {
        optional_object(inputs, "schema")?;
    }
    let object =
        lightflow::serde_json::from_str::<Value>(text).unwrap_or_else(|_| json!({"text": text}));
    Ok((
        Map::from_iter([
            ("json".to_owned(), Value::String(object.to_string())),
            ("object".to_owned(), object),
        ]),
        Vec::new(),
    ))
}

fn render_template(template: &str, vars: Option<&Map<String, Value>>) -> String {
    let mut rendered = String::new();
    let mut rest = template;
    let vars = vars.cloned().map(Value::Object);
    while let Some(start) = rest.find("{{") {
        rendered.push_str(&rest[..start]);
        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find("}}") else {
            rendered.push_str(&rest[start..]);
            return rendered;
        };
        let key = after_start[..end].trim();
        if let Some(value) = vars.as_ref().and_then(|vars| lookup_json_path(vars, key)) {
            rendered.push_str(&json_text(value));
        }
        rest = &after_start[end + 2..];
    }
    rendered.push_str(rest);
    rendered
}

fn lookup_json_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let path = path.trim().strip_prefix('$').unwrap_or(path.trim());
    let path = path.strip_prefix('.').unwrap_or(path);
    if path.is_empty() {
        return Some(value);
    }
    let mut current = value;
    for segment in path.split('.').filter(|segment| !segment.is_empty()) {
        current = match current {
            Value::Object(map) => map.get(segment)?,
            Value::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}

fn json_text(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        Value::Null => String::new(),
        value => value.to_string(),
    }
}
