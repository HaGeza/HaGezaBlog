use macroquad::math::{Vec2, vec2};
use serde_yaml::Value;
use std::f32::consts::FRAC_PI_4;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

fn _parse_f32(value: &Value) -> f32 {
    value.as_f64().unwrap_or(CameraWControlsConfig::default().longitude as f64) as f32
}

fn _parse_vec2(value: &Value) -> Vec2 {
    vec2(_parse_f32(&value["x"]), _parse_f32(&value["y"]))
}

fn _load_config() -> Result<Value, Box<dyn std::error::Error>> {
    #[cfg(target_arch = "wasm32")]
    let config_content = include_str!("../config.yaml").to_string();

    #[cfg(not(target_arch = "wasm32"))]
    let config_content = fs::read_to_string("config.yaml")?;

    Ok(serde_yaml::from_str::<Value>(&config_content)?)
}

pub struct CameraWControlsConfig {
    pub longitude: f32,
    pub latitude: f32,
    pub radius: f32,
    pub orbit_sensitivity: Vec2,
    pub pan_sensitivity: Vec2,
    pub zoom_sensitivity: f32,
    pub touch_zoom_threshold: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for CameraWControlsConfig {
    fn default() -> Self {
        CameraWControlsConfig {
            longitude: 0.0,
            latitude: FRAC_PI_4,
            radius: 5.0,
            orbit_sensitivity: vec2(0.5, 0.5),
            pan_sensitivity: vec2(1.0, 1.0),
            zoom_sensitivity: 0.05,
            touch_zoom_threshold: 0.01,
            z_near: 0.0001,
            z_far: 1000.0,
        }
    }
}

pub fn load_camera_config() -> Result<CameraWControlsConfig, Box<dyn std::error::Error>> {
    let camera = &_load_config()?["camera"];

    Ok(CameraWControlsConfig {
        longitude: _parse_f32(&camera["longitude"]),
        latitude: _parse_f32(&camera["latitude"]),
        radius: _parse_f32(&camera["radius"]),
        orbit_sensitivity: _parse_vec2(&camera["orbit_sensitivity"]),
        pan_sensitivity: _parse_vec2(&camera["pan_sensitivity"]),
        zoom_sensitivity: _parse_f32(&camera["zoom_sensitivity"]),
        touch_zoom_threshold: _parse_f32(&camera["touch_zoom_threshold"]),
        z_near: _parse_f32(&camera["z_near"]),
        z_far: _parse_f32(&camera["z_far"]),
    })
}
