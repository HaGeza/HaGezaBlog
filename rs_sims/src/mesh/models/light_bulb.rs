use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use macroquad::{
    math::{Vec2, vec2},
    models::Mesh,
};

use crate::shape::common::get_semicircle;
use crate::{
    mesh::{common::combine_meshes, lathe_mesh::create_lathe_mesh},
    shape::common::intersect,
};

enum LightBulbGlassProfileGenerationError {
    TooFewSemicircleSection,
    TooSmallSemicircleAngle,
    TooLargeSemicircleAngle,
}

const fn create_light_bulb_glass_profile(
    semicircle_sections: usize,
    semicircle_angle: f32,
    semicircle_radius: f32,
    cap_half_width: f32,
) -> Result<Vec<Vec2>, LightBulbGlassProfileGenerationError> {
    if semicircle_sections < 2 {
        return Err(LightBulbGlassProfileGenerationError::TooFewSemicircleSection);
    }
    if semicircle_angle <= FRAC_PI_2 {
        return Err(LightBulbGlassProfileGenerationError::TooSmallSemicircleAngle);
    }
    if semicircle_angle >= PI {
        return Err(LightBulbGlassProfileGenerationError::TooSmallSemicircleAngle);
    }

    let glass_semicircle: Vec<Vec2> =
        get_semicircle(semicircle_radius, FRAC_PI_2 - semicircle_angle, FRAC_PI_2, semicircle_sections);

    if glass_semicircle[0].x <= 0.
        || glass_semicircle[0].y >= 0.
        || glass_semicircle[1].x <= 0.
        || glass_semicircle[1].y >= 0.
    {
        // The last section of the glass bulb profile should be in the fourth quarter.
        // If it isn't and `theta` is in the correct range, the resolution is not high enough
        return Err(LightBulbGlassProfileGenerationError::TooFewSemicircleSection);
    }

    let glass_curve_a = glass_semicircle[1];
    let glass_line = [&glass_curve_a, &glass_semicircle[0]];
    let cap_line = [&vec2(cap_half_width, 0.), &vec2(cap_half_width, 1.)];
    let Some(glass_curve_b) = intersect(glass_line, cap_line) else {
        panic!("");
    };

    let glass_curve_c = vec2(glass_curve_b.x, glass_curve_b.y - glass_curve_a.distance(glass_curve_b));

    let glass_curve: Vec<Vec2> = get_quadratic_bezier(glass_curve_a, glass_curve_b, glass_curve_c);

    Ok([glass_curve, glass_semicircle[1..]].concat())
}

fn create_light_bulb_cap_profile(glass_bottom_y: f32, cap_half_width: f32, cap_height: f32) -> Vec<Vec2> {
    vec![
        vec2(cap_half_width, glass_bottom_y),
        vec2(cap_half_width, glass_bottom_y - cap_height),
    ]
}

pub fn create_light_bulb_mesh(light_bulb_num_rings: u16) -> Mesh {
    let glass_profile = create_light_bulb_glass_profile();
    let cap_profile = create_light_bulb_cap_profile(glass_profile[0]);

    combine_meshes(&create_lathe_mesh(&glass_profile, light_bulb_num_rings), &create_lathe_mesh(&cap_profile, 2))
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
