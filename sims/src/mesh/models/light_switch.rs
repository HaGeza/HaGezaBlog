use std::{f32::consts::FRAC_PI_2, ops::Add, path::Path};

use macroquad::{
    color::Color,
    math::{Mat4, Vec2, Vec3, vec2, vec3},
};
use serde::Deserialize;
use thiserror::Error;

use crate::{
    mesh::{
        extrusion_mesh::extrude_profile,
        frag_feature::FragFeatures,
        material::{MaterialBuilder, MaterialBuilderError},
        models::model::{MeshWMaterial, draw_model},
        plane::{Plane, XZ_PLANE},
    },
    shape::{
        common::{get_rectangle, get_semicircle, into_circle},
        profile::Profile,
    },
    util::config::{ConfigLoadError, deserialize_color, deserialize_vec2, deserialize_vec3, load_config},
};

#[derive(Deserialize)]
struct LightSwitchFrameParams {
    #[serde(deserialize_with = "deserialize_vec2")]
    frame_dims: Vec2,
    #[serde(deserialize_with = "deserialize_vec2")]
    hole_dims: Vec2,
    height: f32,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
}

#[derive(Deserialize)]
struct LightSwitchToggleParams {
    base_radians: f32,
    base_radius: f32,
    base_num_sections: usize,
    tip_length: f32,
    thickness: f32,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
}

#[derive(Deserialize)]
struct LightSwitchParams {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    frame: LightSwitchFrameParams,
    toggle: LightSwitchToggleParams,
}

#[derive(Error, Debug)]
pub enum LightSwitchProfileError {
    #[error("frame ane hole dimensions must be positive")]
    NonPositiveFrameDims,
    #[error("toggle radius, radians and length must be positive")]
    NonPositiveToggleAttrs,
    #[error("frame must be larger than hole in both dimensions")]
    TooSmallFrame,
}

#[derive(Error, Debug)]
pub enum LightSwitchError {
    #[error("failed to create profile: {0}")]
    Profile(#[from] LightSwitchProfileError),
    #[error("failed to build material: {0}")]
    Material(#[from] MaterialBuilderError),
    #[error("failed to load config: {0}")]
    Config(#[from] ConfigLoadError),
}

pub struct LightSwitch {
    frame: MeshWMaterial,
    toggle: MeshWMaterial,
    model_matrix: Mat4,
}

impl LightSwitch {
    fn new(params: LightSwitchParams, scene_frags: &FragFeatures) -> Result<LightSwitch, LightSwitchError> {
        let mut builder = MaterialBuilder::new();
        builder.add_frag_features(scene_frags.get_frags())?;

        // TODO: move color to config
        Ok(LightSwitch {
            frame: MeshWMaterial::basic(
                extrude_profile(
                    &Self::create_frame_profile(&params.frame)?,
                    &XZ_PLANE,
                    params.frame.height,
                    &params.frame.color,
                ),
                builder.build()?,
            ),
            toggle: MeshWMaterial::basic(
                extrude_profile(
                    &Self::create_toggle_profile(&params.toggle)?,
                    &Plane::new(vec3(-params.toggle.thickness / 2., 0., 0.), Vec3::X),
                    params.toggle.thickness,
                    &params.toggle.color,
                ),
                builder.build()?,
            ),
            model_matrix: Mat4::from_translation(params.position),
        })
    }

    fn create_frame_profile(params: &LightSwitchFrameParams) -> Result<Profile, LightSwitchProfileError> {
        if params.frame_dims.x <= 0.
            || params.frame_dims.y <= 0.
            || params.hole_dims.x <= 0.
            || params.hole_dims.y <= 0.
        {
            Err(LightSwitchProfileError::NonPositiveFrameDims)
        } else if params.frame_dims.x <= params.hole_dims.x || params.frame_dims.y <= params.hole_dims.y {
            Err(LightSwitchProfileError::TooSmallFrame)
        } else {
            Ok(Profile::new(
                get_rectangle(&params.frame_dims).to_vec(),
                vec![get_rectangle(&params.hole_dims).to_vec()],
            ))
        }
    }

    fn create_toggle_profile(params: &LightSwitchToggleParams) -> Result<Profile, LightSwitchProfileError> {
        if params.base_radians <= 0. || params.base_radius <= 0. || params.tip_length <= 0. {
            Err(LightSwitchProfileError::NonPositiveToggleAttrs)
        } else {
            let theta = into_circle(params.base_radians) / 2.;
            let base =
                get_semicircle(params.base_radius, 3. * FRAC_PI_2 - theta, theta - FRAC_PI_2, params.base_num_sections);

            let tip_vec = vec2(0., params.tip_length);
            let (tip_left, tip_right) = (base[0].add(tip_vec), base[base.len() - 1].add(tip_vec));

            Ok(Profile::whole(vec![tip_left].into_iter().chain(base).chain(vec![tip_right]).collect()))
        }
    }

    pub fn from_config(config_path: &Path, scene_frags: &FragFeatures) -> Result<LightSwitch, LightSwitchError> {
        LightSwitch::new(load_config(config_path)?, scene_frags)
    }

    pub fn draw(&self) {
        draw_model(&[&self.frame, &self.toggle], &self.model_matrix)
    }
}
