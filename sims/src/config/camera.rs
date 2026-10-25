use macroquad::math::Vec2;
use serde_yaml::Value;

use crate::config::common::{ConfigLoaderError, parse_f32, parse_vec2};

pub struct CameraWControlsConfig {
    pub longitude: f32,
    pub latitude: f32,
    pub radius: f32,
    pub rotate_mouse_sensitivity: Vec2,
    pub pan_mouse_sensitivity: Vec2,
    pub zoom_mouse_sensitivity: f32,
    pub rotate_touch_sensitivity: Vec2,
    pub pan_touch_sensitivity: Vec2,
    pub zoom_touch_sensitivity: f32,
    pub touch_zoom_threshold: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl CameraWControlsConfig {
    pub fn load(config_path: &str) -> Result<Self, ConfigLoaderError> {
        let yaml: Value = serde_yaml::from_reader(std::fs::File::open(config_path)?)?;

        Ok(Self {
            longitude: parse_f32(&yaml, &["longitude"])?,
            latitude: parse_f32(&yaml, &["latitude"])?,
            radius: parse_f32(&yaml, &["radius"])?,
            rotate_mouse_sensitivity: parse_vec2(&yaml, &["rotate_mouse_sensitivity"])?,
            pan_mouse_sensitivity: parse_vec2(&yaml, &["pan_mouse_sensitivity"])?,
            zoom_mouse_sensitivity: parse_f32(&yaml, &["zoom_mouse_sensitivity"])?,
            rotate_touch_sensitivity: parse_vec2(&yaml, &["rotate_touch_sensitivity"])?,
            pan_touch_sensitivity: parse_vec2(&yaml, &["pan_touch_sensitivity"])?,
            zoom_touch_sensitivity: parse_f32(&yaml, &["zoom_touch_sensitivity"])?,
            touch_zoom_threshold: parse_f32(&yaml, &["touch_zoom_threshold"])?,
            z_near: parse_f32(&yaml, &["z_near"])?,
            z_far: parse_f32(&yaml, &["z_far"])?,
        })
    }
}
