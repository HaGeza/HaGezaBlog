use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use macroquad::{
    color::{self, Color},
    material::{Material, gl_use_default_material, gl_use_material},
    math::{Vec2, vec2, vec3},
    models::{Mesh, draw_mesh},
};
use thiserror::Error;

use crate::{
    mesh::material::{FragFeature, MaterialBuilderError, draw_transparent_mesh},
    shape::{
        common::{get_quadratic_bezier, get_semicircle},
        lathe_profile::{LatheProfile, LatheProfileError},
    },
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

struct LightBulbTopProfileParams {
    head_num_sections: usize,
    head_semicircle_radians: f32,
    head_semicircle_radius: f32,
    neck_num_sections: usize,
}

#[derive(Error, Debug)]
enum LightBulbProfileError {
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

struct LightBulbBottomProfileParams {
    start_y: f32,
    half_width: f32,
    height: f32,
}

struct LightBulbParams {
    top_profile_params: LightBulbTopProfileParams,
    bottom_profile_params: LightBulbBottomProfileParams,
    num_rings: usize,
}

#[derive(Error, Debug)]
enum LightBulbError {
    #[error("failed to create profile: {0}")]
    ProfileError(#[from] LightBulbProfileError),
    #[error("failed to build material: {0}")]
    MaterialError(#[from] MaterialBuilderError),
}

pub struct LightBulb {
    bottom: Mesh,
    top_outer: Mesh,
    //top_inner: Mesh
    bottom_material: Material,
    top_material: Material,
}

impl LightBulb {
    fn new(params: LightBulbParams) -> Result<Self, LightBulbError> {
        let bottom_profile = Self::create_bottom_profile(params.bottom_profile_params)?;
        let top_profile = Self::create_top_profile(params.top_profile_params, &bottom_profile)?;

        let mut builder = MaterialBuilder::new();
        builder
            .add_frag_features(&[FragFeature::AmbientLight(0.2), FragFeature::DirectionalLight(vec3(1., 0., 0.))])?;

        Ok(LightBulb {
            bottom: create_lathe_mesh(&bottom_profile, params.num_rings, &color::RED, false),
            top_outer: create_lathe_mesh(&top_profile, params.num_rings, &Color::new(1., 1., 1., 0.4), false),
            bottom_material: builder.set_transparency(false).build()?,
            top_material: builder.set_transparency(true).build()?,
        })
    }

    pub fn draw(&self) {
        gl_use_material(&self.bottom_material);
        draw_mesh(&self.bottom);
        gl_use_material(&self.top_material);
        unsafe {
            draw_transparent_mesh(&self.top_outer);
        }
        gl_use_default_material();
    }

    fn create_bottom_profile(params: LightBulbBottomProfileParams) -> Result<LatheProfile, LightBulbProfileError> {
        Ok(LatheProfile::new(&[
            vec2(0., params.start_y),
            vec2(params.half_width, params.start_y),
            vec2(params.half_width, params.start_y + params.height),
            vec2(0., params.start_y + params.height),
        ])?)
    }

    fn create_top_profile(
        params: LightBulbTopProfileParams,
        bottom_profile: &LatheProfile,
    ) -> Result<LatheProfile, LightBulbProfileError> {
        if params.head_num_sections < 2 {
            return Err(LightBulbProfileError::TooFewSections);
        }
        if params.head_semicircle_radians <= FRAC_PI_2 {
            return Err(LightBulbProfileError::TooSmallAngle);
        }
        if params.head_semicircle_radians >= PI {
            return Err(LightBulbProfileError::TooLargeAngle);
        }

        let head_semicircle: Vec<Vec2> = get_semicircle(
            params.head_semicircle_radius,
            FRAC_PI_2 - params.head_semicircle_radians,
            FRAC_PI_2,
            params.head_num_sections,
        );

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

        let mut neck: Vec<Vec2> = get_quadratic_bezier([neck_a, &neck_b, &neck_c], params.neck_num_sections);
        neck[0].y += 0.01;

        let points: Vec<Vec2> = vec![vec2(0., neck[0].y)]
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
}

const HEAD_SEMICIRCLE_DEFAULT_RADIUS: f32 = 2.;
const BOTTOM_DEFAULT_HALF_WIDTH: f32 = HEAD_SEMICIRCLE_DEFAULT_RADIUS / 2.5;
const BOTTOM_DEFAULT_HEIGHT: f32 = BOTTOM_DEFAULT_HALF_WIDTH;
const BOTTOM_DEFAULT_START_Y: f32 = -HEAD_SEMICIRCLE_DEFAULT_RADIUS * 1.25 - BOTTOM_DEFAULT_HEIGHT;

const HEAD_DEFAULT_NUM_SECTIONS: usize = 20;
const HEAD_SEMICIRCLE_DEFAULT_RADIANS: f32 = 3. * FRAC_PI_4;
const NECK_DEFAULT_NUM_SECTIONS: usize = HEAD_DEFAULT_NUM_SECTIONS / 5;
const NUM_RINGS: usize = HEAD_DEFAULT_NUM_SECTIONS + NECK_DEFAULT_NUM_SECTIONS;

pub fn get_light_bulb() -> LightBulb {
    LightBulb::new(LightBulbParams {
        top_profile_params: LightBulbTopProfileParams {
            head_num_sections: HEAD_DEFAULT_NUM_SECTIONS,
            head_semicircle_radians: HEAD_SEMICIRCLE_DEFAULT_RADIANS,
            head_semicircle_radius: HEAD_SEMICIRCLE_DEFAULT_RADIUS,
            neck_num_sections: NECK_DEFAULT_NUM_SECTIONS,
        },
        bottom_profile_params: LightBulbBottomProfileParams {
            start_y: BOTTOM_DEFAULT_START_Y,
            half_width: BOTTOM_DEFAULT_HALF_WIDTH,
            height: BOTTOM_DEFAULT_HEIGHT,
        },
        num_rings: NUM_RINGS,
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_light_bulb_top_mesh_works() {}

    #[test]
    fn test_create_light_bulb_bottom_mesh_works() {}

    #[test]
    fn test_create_light_bulb_mesh_works() {}
}
