use std::{io, path::Path};

use macroquad::{
    color::Color,
    math::{Vec2, Vec3, vec2, vec3, vec4},
};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Deserializer error: {0}")]
    TomlError(#[from] toml::de::Error),
}

pub fn load_config<D>(config_path: &Path) -> Result<D, ConfigLoadError>
where
    D: DeserializeOwned,
{
    Ok(toml::from_str(&std::fs::read_to_string(config_path)?)?)
}

pub fn deserialize_vec2<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y] = <[f32; 2]>::deserialize(deserializer)?;
    Ok(vec2(x, y))
}

pub fn deserialize_vec3<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y, z] = <[f32; 3]>::deserialize(deserializer)?;
    Ok(vec3(x, y, z))
}

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y, z, w] = <[f32; 4]>::deserialize(deserializer)?;
    Ok(Color::new(x, y, z, w))
}
