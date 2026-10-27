use std::{io, path::Path};

use macroquad::math::{Vec2, Vec3, vec2, vec3};
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
    #[derive(Deserialize)]
    struct Vec2Components {
        x: f32,
        y: f32,
    }

    let components = Vec2Components::deserialize(deserializer)?;
    Ok(vec2(components.x, components.y))
}

pub fn deserialize_vec3<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Vec3Components {
        x: f32,
        y: f32,
        z: f32,
    }

    let components = Vec3Components::deserialize(deserializer)?;
    Ok(vec3(components.x, components.y, components.z))
}
