use std::{
    f32::consts::{FRAC_PI_2, PI},
    path::Path,
};

use macroquad::{
    color::Color,
    math::{Mat4, Vec2, Vec3, vec2, vec3},
};
use serde::Deserialize;
use thiserror::Error;

use crate::{
    mesh::{
        frag_feature::{FragFeature, FragFeatures},
        material::MaterialBuilderError,
        models::model::{MeshWMaterial, draw_model},
    },
    shape::{
        common::{get_quadratic_bezier, get_semicircle},
        lathe_profile::{LatheProfile, LatheProfileError},
    },
    util::config::{ConfigLoadError, deserialize_color, deserialize_vec3, load_config},
};
use crate::{
    mesh::{lathe_mesh::create_lathe_mesh, material::MaterialBuilder},
    shape::common::intersect,
};

//          ******          +---+------+
//       ************           |      |
//     ****************         |      |
//    ******************        | HEAD |
//    ******************        |      |
//    ******************        |      |
//     ****************         |      |
//      **************      +---+      | TOP
//       ************           |      |
//       ************           | NECK |
//        **********            |      |
//        **********            |      |
//        **********        +---+------+
//        **********            |
//        **********            | BOTTOM
//         ********             |
//          ******              |
//           ****           +---+

#[derive(Error, Debug)]
pub enum LightBulbProfileError {
    #[error("too few head sections, cannot create semicircle")]
    TooFewSections,
    #[error("too small head semicircle radian, head and bottom profiles wouldn't intersect below head")]
    TooSmallAngle,
    #[error("too large head semicircle radian, semicircle wouldn't fit in profile")]
    TooLargeAngle,
    #[error("top and bottom border sections don't intersect")]
    NoIntersection,
    #[error("incorrect lathe profile: {0}")]
    IncorrectProfile(#[from] LatheProfileError),
}

#[derive(Deserialize)]
struct LightBulbTopParams {
    head_num_sections: usize,
    head_radians: f32,
    head_radius: f32,
    neck_num_sections: usize,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
}

#[derive(Deserialize)]
struct LightBulbBottomParams {
    start_y: f32,
    half_width: f32,
    height: f32,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
}

#[derive(Deserialize)]
struct LightBulbParams {
    #[serde(deserialize_with = "deserialize_vec3")]
    position: Vec3,
    #[serde(deserialize_with = "deserialize_vec3")]
    point_light_color: Vec3,
    num_rings: usize,
    top: LightBulbTopParams,
    bottom: LightBulbBottomParams,
}

#[derive(Error, Debug)]
pub enum LightBulbError {
    #[error("failed to create profile: {0}")]
    ProfileError(#[from] LightBulbProfileError),
    #[error("failed to build material: {0}")]
    MaterialError(#[from] MaterialBuilderError),
    #[error("failed to load config: {0}")]
    ConfigError(#[from] ConfigLoadError),
}

pub struct LightBulb {
    bottom: MeshWMaterial,
    top_inner: MeshWMaterial,
    top_outer: MeshWMaterial,
    model_matrix: Mat4,
}

impl LightBulb {
    fn new(params: LightBulbParams, scene_frags: &FragFeatures) -> Result<Self, LightBulbError> {
        let bottom_profile = Self::create_bottom_profile(&params.bottom)?;
        let top_outer_profile = Self::create_top_outer_profile(&params.top, &bottom_profile)?;
        let top_inner_profile = top_outer_profile.scale(0.98);

        let model_matrix = Mat4::from_translation(params.position);

        let point_light_y =
            top_outer_profile.points().iter().map(|pt| pt.y).sum::<f32>() / top_outer_profile.points().len() as f32;
        let point_light_pos = model_matrix.transform_point3(vec3(0., point_light_y, 0.));
        let point_light = FragFeature::PointLight { position: point_light_pos, color: params.point_light_color };

        let mut builder = MaterialBuilder::new();
        builder.add_frag_feature(&point_light)?;

        Ok(LightBulb {
            bottom: MeshWMaterial::basic(
                create_lathe_mesh(&bottom_profile, params.num_rings, &params.bottom.color, false),
                builder.clone().add_frag_features(scene_frags.get_frags())?.build()?,
            ),
            top_inner: MeshWMaterial::new(
                create_lathe_mesh(&top_outer_profile, params.num_rings, &params.top.color, true),
                builder.clone().set_transparency(true).add_frag_features(scene_frags.get_frags())?.build()?,
                true,
            ),
            top_outer: MeshWMaterial::new(
                create_lathe_mesh(&top_inner_profile, params.num_rings, &params.top.color, false),
                builder
                    .clone()
                    .set_transparency(true)
                    .add_frag_features(scene_frags.scale_lights(1. - params.top.color.a).get_frags())?
                    .build()?,
                true,
            ),
            model_matrix,
        })
    }

    fn create_bottom_profile(params: &LightBulbBottomParams) -> Result<LatheProfile, LightBulbProfileError> {
        Ok(LatheProfile::new(&[
            vec2(0., params.start_y),
            vec2(params.half_width, params.start_y),
            vec2(params.half_width, params.start_y + params.height),
            vec2(0., params.start_y + params.height),
        ])?)
    }

    fn create_top_outer_profile(
        params: &LightBulbTopParams,
        bottom_profile: &LatheProfile,
    ) -> Result<LatheProfile, LightBulbProfileError> {
        if params.head_num_sections < 2 {
            return Err(LightBulbProfileError::TooFewSections);
        }
        if params.head_radians <= FRAC_PI_2 {
            return Err(LightBulbProfileError::TooSmallAngle);
        }
        if params.head_radians >= PI {
            return Err(LightBulbProfileError::TooLargeAngle);
        }

        let head_semicircle: Vec<Vec2> =
            get_semicircle(params.head_radius, FRAC_PI_2 - params.head_radians, FRAC_PI_2, params.head_num_sections);

        if head_semicircle[0].y >= 0. || head_semicircle[1].y >= 0. {
            // The last section of the glass bulb profile should be in the fourth quarter.
            // If it isn't and `theta` is in the correct range, the resolution is not high enough
            return Err(LightBulbProfileError::TooFewSections);
        }

        let neck_a = bottom_profile.point(bottom_profile.len() - 2);
        let neck_c = head_semicircle[0];

        let head_end_section = [&head_semicircle[1], &neck_c];
        let bottom_section =
            [bottom_profile.point(bottom_profile.len() - 3), bottom_profile.point(bottom_profile.len() - 2)];
        let Some(neck_b) = intersect(head_end_section, bottom_section) else {
            return Err(LightBulbProfileError::NoIntersection);
        };

        let neck: Vec<Vec2> = get_quadratic_bezier([neck_a, &neck_b, &neck_c], params.neck_num_sections);

        let points: Vec<Vec2> = vec![vec2(0.0, neck[0].y)]
            .into_iter()
            .chain(neck)
            .chain(head_semicircle.into_iter().skip(1))
            .map(|mut pt| {
                pt.x = pt.x.max(0.);
                pt
            })
            .collect();

        Ok(LatheProfile::new(&points)?)
    }

    pub fn from_config(config_path: &Path, scene_frags: &FragFeatures) -> Result<LightBulb, LightBulbError> {
        LightBulb::new(load_config(config_path)?, scene_frags)
    }

    pub fn draw(&self) {
        draw_model(&[&self.bottom, &self.top_inner, &self.top_outer], &self.model_matrix)
    }
}
