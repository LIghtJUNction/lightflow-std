use crate::RuntimeError;
use lightflow::serde_json::{Map, Value};

pub(crate) fn required<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<&'a Value, RuntimeError> {
    inputs
        .get(name)
        .ok_or_else(|| RuntimeError::new(format!("required input `{name}` is missing")))
}

pub(crate) fn required_string<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<&'a str, RuntimeError> {
    required(inputs, name)?
        .as_str()
        .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON string")))
}

pub(crate) fn optional_string<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<Option<&'a str>, RuntimeError> {
    inputs
        .get(name)
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON string")))
        })
        .transpose()
}

pub(crate) fn required_bool(inputs: &Map<String, Value>, name: &str) -> Result<bool, RuntimeError> {
    required(inputs, name)?
        .as_bool()
        .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON boolean")))
}

pub(crate) fn required_array<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<&'a [Value], RuntimeError> {
    required(inputs, name)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON array")))
}

pub(crate) fn optional_array<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<Option<&'a [Value]>, RuntimeError> {
    inputs
        .get(name)
        .map(|value| {
            value
                .as_array()
                .map(Vec::as_slice)
                .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON array")))
        })
        .transpose()
}

pub(crate) fn required_object<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<&'a Map<String, Value>, RuntimeError> {
    required(inputs, name)?
        .as_object()
        .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON object")))
}

pub(crate) fn optional_object<'a>(
    inputs: &'a Map<String, Value>,
    name: &str,
) -> Result<Option<&'a Map<String, Value>>, RuntimeError> {
    inputs
        .get(name)
        .map(|value| {
            value
                .as_object()
                .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a JSON object")))
        })
        .transpose()
}

pub(crate) fn required_u32(inputs: &Map<String, Value>, name: &str) -> Result<u32, RuntimeError> {
    let value = required(inputs, name)?
        .as_u64()
        .ok_or_else(|| RuntimeError::new(format!("input `{name}` must be a positive integer")))?;
    u32::try_from(value)
        .map_err(|_| RuntimeError::new(format!("input `{name}` is larger than u32")))
}

pub(crate) fn optional_u32(
    inputs: &Map<String, Value>,
    name: &str,
) -> Result<Option<u32>, RuntimeError> {
    inputs
        .get(name)
        .map(|_| required_u32(inputs, name))
        .transpose()
}

pub(crate) fn optional_u64(
    inputs: &Map<String, Value>,
    name: &str,
) -> Result<Option<u64>, RuntimeError> {
    inputs
        .get(name)
        .map(|value| {
            value.as_u64().ok_or_else(|| {
                RuntimeError::new(format!("input `{name}` must be a non-negative integer"))
            })
        })
        .transpose()
}
