use crate::RuntimeError;
use crate::input::{
    optional_string, required, required_array, required_bool, required_object, required_string,
};
use crate::text::Output;
use lightflow::serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;

pub(crate) fn control_if(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let condition = required_bool(inputs, "condition")?;
    let (name, key) = if condition {
        ("then", "then_value")
    } else {
        ("else", "else_value")
    };
    Ok((
        Map::from_iter([
            ("selected".to_owned(), Value::String(name.to_owned())),
            (
                "value".to_owned(),
                inputs.get(key).cloned().unwrap_or(Value::Null),
            ),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn control_switch(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let selector = required_string(inputs, "selector")?;
    let cases = required_object(inputs, "cases")?;
    let (value, selected) = cases
        .get(selector)
        .map(|value| (value.clone(), selector))
        .unwrap_or_else(|| {
            (
                inputs.get("default").cloned().unwrap_or(Value::Null),
                "default",
            )
        });
    Ok((
        Map::from_iter([
            ("selected".to_owned(), Value::String(selected.to_owned())),
            ("value".to_owned(), value),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn control_merge(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let mode = optional_string(inputs, "mode")?.unwrap_or("first_non_null");
    let a = inputs.get("a").cloned().unwrap_or(Value::Null);
    let b = inputs.get("b").cloned().unwrap_or(Value::Null);
    let value = match mode {
        "object" => {
            let mut merged = Map::new();
            if let Value::Object(map) = a {
                merged.extend(map);
            } else if !a.is_null() {
                return Err(RuntimeError::new(
                    "input `a` must be a JSON object in object mode",
                ));
            }
            if let Value::Object(map) = b {
                merged.extend(map);
            } else if !b.is_null() {
                return Err(RuntimeError::new(
                    "input `b` must be a JSON object in object mode",
                ));
            }
            Value::Object(merged)
        }
        "array" => json!([a, b]),
        "first_non_null" => {
            if a.is_null() {
                b
            } else {
                a
            }
        }
        other => {
            return Err(RuntimeError::new(format!(
                "unsupported merge mode: {other}"
            )));
        }
    };
    Ok((
        Map::from_iter([
            ("selected".to_owned(), Value::String(mode.to_owned())),
            ("value".to_owned(), value),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn control_split(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let value = required(inputs, "value")?.clone();
    let (first, rest, items) = match value {
        Value::Array(items) => (
            items.first().cloned().unwrap_or(Value::Null),
            Value::Array(items.iter().skip(1).cloned().collect()),
            Value::Array(items),
        ),
        Value::Object(map) => {
            let items = map
                .iter()
                .map(|(key, value)| json!({"key": key, "value": value}))
                .collect::<Vec<_>>();
            (
                items.first().cloned().unwrap_or(Value::Null),
                Value::Array(items.iter().skip(1).cloned().collect()),
                Value::Array(items),
            )
        }
        other => (other.clone(), Value::Null, json!([other])),
    };
    Ok((
        Map::from_iter([
            ("first".to_owned(), first),
            ("items".to_owned(), items),
            ("rest".to_owned(), rest),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn model_select(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let requirement_id = optional_string(inputs, "requirement_id")?.unwrap_or_default();
    let preferred = optional_string(inputs, "preferred")?;
    let variants = required_array(inputs, "variants")?;
    for (index, variant) in variants.iter().enumerate() {
        if !variant.is_object() {
            return Err(RuntimeError::new(format!(
                "input `variants[{index}]` must be a JSON object"
            )));
        }
    }
    let selected = preferred
        .and_then(|preferred| {
            variants.iter().find(|variant| {
                variant.get("id").and_then(Value::as_str) == Some(preferred)
                    || variant.get("format").and_then(Value::as_str) == Some(preferred)
            })
        })
        .or_else(|| variants.first())
        .cloned()
        .unwrap_or(Value::Null);
    let variant_id = selected
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    Ok((
        Map::from_iter([
            ("model".to_owned(), selected),
            (
                "requirement_id".to_owned(),
                Value::String(requirement_id.to_owned()),
            ),
            ("variant_id".to_owned(), Value::String(variant_id)),
        ]),
        Vec::new(),
    ))
}

pub(crate) fn model_lock_check(inputs: &Map<String, Value>) -> Result<Output, RuntimeError> {
    let workflow_id = required_string(inputs, "workflow_id")?;
    let requirement_id = required_string(inputs, "requirement_id")?;
    let key = format!("{workflow_id}::{requirement_id}");
    let lock = if Path::new("lfw.lock").is_file() {
        lightflow::serde_json::from_slice::<Value>(&fs::read("lfw.lock")?)
            .map_err(|error| RuntimeError::new(format!("invalid lfw.lock: {error}")))?
    } else {
        Value::Null
    };
    let entry = lock
        .get("models")
        .and_then(|models| models.get(&key))
        .cloned()
        .unwrap_or(Value::Null);
    let path = entry
        .get("local_paths")
        .and_then(Value::as_array)
        .and_then(|paths| paths.first())
        .cloned()
        .unwrap_or(Value::String(String::new()));
    if !path.is_string() {
        return Err(RuntimeError::new(
            "lfw.lock model local_paths[0] must be a string",
        ));
    }
    let exists = path
        .as_str()
        .is_some_and(|path| !path.is_empty() && Path::new(path).exists());
    // An unlocked requirement has no path; report null so the declared
    // `path`-typed output stays valid.
    let path = match path.as_str() {
        Some("") | None => Value::Null,
        Some(_) => path,
    };
    Ok((
        Map::from_iter([
            ("entry".to_owned(), entry.clone()),
            ("exists".to_owned(), Value::Bool(exists)),
            ("locked".to_owned(), Value::Bool(!entry.is_null())),
            ("path".to_owned(), path),
        ]),
        Vec::new(),
    ))
}
