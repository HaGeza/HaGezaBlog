use std::io;

use macroquad::math::{Vec2, Vec3, vec2, vec3};
use serde_yaml::Value;
use thiserror::Error;

use crate::config::common::ConfigLoaderError::MissingKey;

#[derive(Debug, Error)]
pub enum ConfigLoaderError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_yaml::Error),
    #[error("key '{0}' missing from config")]
    MissingKey(String),
    #[error("failed to parse {value:?} as {type_str}")]
    FailedConversion { value: Value, type_str: &'static str },
}

fn parse_value<'a>(yaml: &'a Value, path: &[&'static str]) -> Result<&'a Value, ConfigLoaderError> {
    let mut value = yaml;

    for key in path {
        value = value
            .as_mapping()
            .and_then(|map| map.get(Value::String(key.to_string())))
            .ok_or(MissingKey(path.join("::")))?
    }
    Ok(value)
}

fn value_to_f32(value: &Value) -> Result<f32, ConfigLoaderError> {
    if let Some(value) = value.as_f64() {
        Ok(value as f32)
    } else {
        Err(ConfigLoaderError::FailedConversion { value: value.to_owned(), type_str: "f64" })
    }
}

pub fn parse_f32(yaml: &Value, path: &[&'static str]) -> Result<f32, ConfigLoaderError> {
    value_to_f32(parse_value(yaml, path)?)
}

pub fn parse_vec2(yaml: &Value, path: &[&'static str]) -> Result<Vec2, ConfigLoaderError> {
    let vec = parse_value(yaml, path)?;
    Ok(vec2(value_to_f32(&vec["x"])?, value_to_f32(&vec["y"])?))
}

pub fn parse_vec3(yaml: &Value, path: &[&'static str]) -> Result<Vec3, ConfigLoaderError> {
    let vec = parse_value(yaml, path)?;
    Ok(vec3(value_to_f32(&vec["x"])?, value_to_f32(&vec["y"])?, value_to_f32(&vec["z"])?))
}
