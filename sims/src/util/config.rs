use macroquad::file::load_string;
use macroquad::{
    color::Color,
    math::{Vec2, Vec3, vec2, vec3},
};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("IO error: {0}")]
    IOError(#[from] macroquad::Error),
    #[error("Deserializer error: {0}")]
    TomlError(#[from] toml::de::Error),
}

#[cfg(target_arch = "wasm32")]
pub const CONFIGS_BASE: &str = "../configs"; // Relative URL to `app/static/templates/sim_loader.html`

#[cfg(not(target_arch = "wasm32"))]
pub const CONFIGS_BASE: &str = "./configs"; // Relative path to `sims`

pub async fn load_config<D>(config_path: &str) -> Result<D, ConfigLoadError>
where
    D: DeserializeOwned,
{
    Ok(toml::from_str(load_string(config_path).await?.as_str())?)
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
