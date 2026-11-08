use std::{ops::Mul, path::Path};

use crate::util::config::{ConfigLoadError, deserialize_vec3, load_config};
use macroquad::math::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FragFeature {
    AmbientLight {
        #[serde(deserialize_with = "deserialize_vec3")]
        value: Vec3,
    },
    DirectionalLight {
        #[serde(deserialize_with = "deserialize_vec3")]
        direction: Vec3,
        #[serde(deserialize_with = "deserialize_vec3")]
        color: Vec3,
    },
    PointLight {
        #[serde(deserialize_with = "deserialize_vec3")]
        position: Vec3,
        #[serde(deserialize_with = "deserialize_vec3")]
        color: Vec3,
    },
}

#[derive(Debug, Deserialize)]
pub struct FragFeatures {
    frags: Vec<FragFeature>,
}

impl FragFeatures {
    pub fn from_config(config_path: &Path) -> Result<FragFeatures, ConfigLoadError> {
        load_config(config_path)
    }

    pub fn get_frags(&self) -> &Vec<FragFeature> {
        &self.frags
    }

    pub fn scale_lights(&self, scalar: f32) -> FragFeatures {
        FragFeatures {
            frags: self
                .frags
                .iter()
                .map(|frag| match frag {
                    FragFeature::AmbientLight { value } => {
                        FragFeature::AmbientLight { value: value.mul(scalar) }
                    },
                    FragFeature::DirectionalLight { direction, color } => {
                        FragFeature::DirectionalLight { direction: *direction, color: color.mul(scalar) }
                    },
                    FragFeature::PointLight { position, color } => {
                        FragFeature::PointLight { position: *position, color: color.mul(scalar) }
                    },
                })
                .collect(),
        }
    }
}
