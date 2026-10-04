use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use macroquad::{
    color,
    math::{Vec2, vec2},
    models::Mesh,
};
use thiserror::Error;

use crate::shape::{
    common::{get_quadratic_bezier, get_semicircle},
    lathe_profile::{LatheProfile, LatheProfileError},
};
use crate::{mesh::lathe_mesh::create_lathe_mesh, shape::common::intersect};

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
    IncorrectProfile(LatheProfileError),
}

impl From<LatheProfileError> for LightBulbProfileError {
    fn from(lathe_error: LatheProfileError) -> LightBulbProfileError {
        LightBulbProfileError::IncorrectProfile(lathe_error)
    }
}

fn create_light_bulb_top_profile(
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

    let neck_a = bottom_profile.point(bottom_profile.len() - 1);
    let neck_c = head_semicircle[0];

    let head_end_section = [&head_semicircle[1], &neck_c];
    let bottom_section =
        [bottom_profile.point(bottom_profile.len() - 2), bottom_profile.point(bottom_profile.len() - 1)];
    let Some(neck_b) = intersect(head_end_section, bottom_section) else {
        return Err(LightBulbProfileError::NoIntersection);
    };

    let neck: Vec<Vec2> = get_quadratic_bezier([&neck_a, &neck_b, &neck_c], params.neck_num_sections);

    let points: Vec<Vec2> = neck
        .into_iter()
        .chain(head_semicircle.into_iter().skip(1))
        .map(|mut pt| {
            pt.x = pt.x.max(0.);
            pt
        })
        .collect();

    Ok(LatheProfile::new(&points)?)
}

struct LightBulbBottomProfileParams {
    start_y: f32,
    half_width: f32,
    height: f32,
}

fn create_light_bulb_bottom_profile(
    params: LightBulbBottomProfileParams,
) -> Result<LatheProfile, LightBulbProfileError> {
    Ok(LatheProfile::new(&vec![
        vec2(params.half_width, params.start_y),
        vec2(params.half_width, params.start_y + params.height),
    ])?)
}

struct LightBulbMeshParams {
    top_profile_params: LightBulbTopProfileParams,
    bottom_profile_params: LightBulbBottomProfileParams,
    num_rings: usize,
}

#[derive(Error, Debug)]
enum LightBulbMeshError {
    #[error("failed to create profile: {0}")]
    ProfileError(LightBulbProfileError),
}

impl From<LightBulbProfileError> for LightBulbMeshError {
    fn from(profile_error: LightBulbProfileError) -> LightBulbMeshError {
        LightBulbMeshError::ProfileError(profile_error)
    }
}

impl From<LatheProfileError> for LightBulbMeshError {
    fn from(lathe_error: LatheProfileError) -> LightBulbMeshError {
        LightBulbMeshError::ProfileError(LightBulbProfileError::IncorrectProfile(lathe_error))
    }
}

fn create_light_bulb_mesh(params: LightBulbMeshParams) -> Result<Mesh, LightBulbMeshError> {
    let bottom_profile = create_light_bulb_bottom_profile(params.bottom_profile_params)?;
    let top_profile = create_light_bulb_top_profile(params.top_profile_params, &bottom_profile)?;
    Ok(create_lathe_mesh(
        &(LatheProfile::new(&[bottom_profile.points(), &top_profile.points()[1..]].concat())?),
        params.num_rings,
        &color::WHITE,
    ))
}

const HEAD_SEMICIRCLE_DEFAULT_RADIUS: f32 = 2.;
const BOTTOM_DEFAULT_HALF_WIDTH: f32 = HEAD_SEMICIRCLE_DEFAULT_RADIUS / 2.5;
const BOTTOM_DEFAULT_HEIGHT: f32 = BOTTOM_DEFAULT_HALF_WIDTH;
const BOTTOM_DEFAULT_START_Y: f32 = -HEAD_SEMICIRCLE_DEFAULT_RADIUS * 1.25 - BOTTOM_DEFAULT_HEIGHT;

const HEAD_DEFAULT_NUM_SECTIONS: usize = 20;
const HEAD_SEMICIRCLE_DEFAULT_RADIANS: f32 = 3. * FRAC_PI_4;
const NECK_DEFAULT_NUM_SECTIONS: usize = HEAD_DEFAULT_NUM_SECTIONS / 5;
const NUM_RINGS: usize = HEAD_DEFAULT_NUM_SECTIONS + NECK_DEFAULT_NUM_SECTIONS;

pub fn get_light_bulb_mesh() -> Mesh {
    create_light_bulb_mesh(LightBulbMeshParams {
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
