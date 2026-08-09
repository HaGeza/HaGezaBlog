use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use macroquad::{
    math::{Vec2, vec2},
    models::Mesh,
};
use thiserror::Error;

use crate::shape::common::{get_quadratic_bezier, get_semicircle};
use crate::{mesh::lathe_mesh::create_lathe_mesh, shape::common::intersect};

struct LightBulbGlassProfileParams {
    glass_num_sections: usize,
    glass_radians: f32,
    glass_radius: f32,
    cap_half_width: f32,
    connection_num_sections: usize,
}

#[derive(Error, Debug)]
enum LightBulbGlassProfileError {
    #[error("too few glass sections, cannot create semicircle")]
    TooFewSections,
    #[error("too small glass angle, glass and cap profiles wouldn't intersect below glass")]
    TooSmallAngle,
    #[error("too large glass angle, semicircle wouldn't fit in profile")]
    TooLargeAngle,
}

fn create_light_bulb_glass_profile(
    params: LightBulbGlassProfileParams,
) -> Result<Vec<Vec2>, LightBulbGlassProfileError> {
    if params.glass_num_sections < 2 {
        return Err(LightBulbGlassProfileError::TooFewSections);
    }
    if params.glass_radians <= FRAC_PI_2 {
        return Err(LightBulbGlassProfileError::TooSmallAngle);
    }
    if params.glass_radians >= PI {
        return Err(LightBulbGlassProfileError::TooLargeAngle);
    }

    let semicircle: Vec<Vec2> =
        get_semicircle(params.glass_radius, FRAC_PI_2 - params.glass_radians, FRAC_PI_2, params.glass_num_sections);

    println!("Got {:?} from Rust", semicircle);

    if semicircle[0].x <= 0. || semicircle[0].y >= 0. || semicircle[1].x <= 0. || semicircle[1].y >= 0. {
        // The last section of the glass bulb profile should be in the fourth quarter.
        // If it isn't and `theta` is in the correct range, the resolution is not high enough
        return Err(LightBulbGlassProfileError::TooFewSections);
    }

    let connection_c = semicircle[1];
    let glass_line = [&connection_c, &semicircle[0]];
    let cap_line = [&vec2(params.cap_half_width, 0.), &vec2(params.cap_half_width, 1.)];
    let Some(connection_b) = intersect(glass_line, cap_line) else {
        panic!("");
    };

    let connection_a = vec2(connection_b.x, connection_b.y - connection_c.distance(connection_b));

    let connection: Vec<Vec2> =
        get_quadratic_bezier(&connection_a, &connection_b, &connection_c, params.connection_num_sections);

    Ok([connection, semicircle[1..].to_vec()].concat())
}

struct LightBulbCapProfileParams {
    glass_bottom_y: f32,
    cap_half_width: f32,
    cap_height: f32,
}

fn create_light_bulb_cap_profile(params: LightBulbCapProfileParams) -> Vec<Vec2> {
    vec![
        vec2(params.cap_half_width, params.glass_bottom_y),
        vec2(params.cap_half_width, params.glass_bottom_y + params.cap_height),
    ]
}

struct LightBulbMeshParams {
    glass_profile_params: LightBulbGlassProfileParams,
    cap_profile_params: LightBulbCapProfileParams,
    num_rings: usize,
}

#[derive(Error, Debug)]
enum LightBulbMeshError {
    #[error("failed to create glass profile: {0}")]
    GlassError(LightBulbGlassProfileError),
}

fn create_light_bulb_mesh(params: LightBulbMeshParams) -> Result<Mesh, LightBulbMeshError> {
    let glass_profile = match create_light_bulb_glass_profile(params.glass_profile_params) {
        Ok(glass_profile) => glass_profile,
        Err(glass_profile_error) => return Err(LightBulbMeshError::GlassError(glass_profile_error)),
    };
    let cap_profile = create_light_bulb_cap_profile(params.cap_profile_params);

    Ok(create_lathe_mesh(&[cap_profile, glass_profile].concat(), params.num_rings))
}

const GLASS_DEFAULT_RADIUS: f32 = 2.;
const CAP_DEFAULT_HALF_WIDTH: f32 = GLASS_DEFAULT_RADIUS / 2.5;
const CAP_DEFAULT_HEIGHT: f32 = CAP_DEFAULT_HALF_WIDTH * 1.5;
const CAP_DEFAULT_BOTTOM: f32 = -GLASS_DEFAULT_RADIUS - CAP_DEFAULT_HEIGHT;

const GLASS_DEFAULT_NUM_SECTIONS: usize = 20;
const GLASS_DEFAULT_RADIANS: f32 = 3. * FRAC_PI_4;
const CONNECTION_DEFAULT_NUM_SECTIONS: usize = GLASS_DEFAULT_NUM_SECTIONS / 5;
const NUM_RINGS: usize = GLASS_DEFAULT_NUM_SECTIONS + CONNECTION_DEFAULT_NUM_SECTIONS;

pub fn get_light_bulb_mesh() -> Mesh {
    create_light_bulb_mesh(LightBulbMeshParams {
        glass_profile_params: LightBulbGlassProfileParams {
            glass_num_sections: GLASS_DEFAULT_NUM_SECTIONS,
            glass_radians: GLASS_DEFAULT_RADIANS,
            glass_radius: GLASS_DEFAULT_RADIUS,
            cap_half_width: CAP_DEFAULT_HALF_WIDTH,
            connection_num_sections: CONNECTION_DEFAULT_NUM_SECTIONS,
        },
        cap_profile_params: LightBulbCapProfileParams {
            glass_bottom_y: CAP_DEFAULT_BOTTOM,
            cap_half_width: CAP_DEFAULT_HALF_WIDTH,
            cap_height: CAP_DEFAULT_HEIGHT,
        },
        num_rings: NUM_RINGS,
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_light_bulb_glass_mesh_works() {}

    #[test]
    fn test_create_light_bulb_cap_mesh_works() {}

    #[test]
    fn test_create_light_bulb_mesh_works() {}
}
