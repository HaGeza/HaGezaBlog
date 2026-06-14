use macroquad::{
    math::{Vec2, vec2},
    models::Mesh,
};

use crate::mesh::{common::combine_meshes, lathe_mesh::create_lathe_mesh};

const GLASS_SEGMENTS_TO_RINGS_RATIO: f32 = 1.;

fn create_light_bulb_glass_profile() -> Vec<Vec2> {
    let semicircle: Vec<Vec2> = get_semicircle();

    let curve_a = semicircle.last()?;
    let curve_b = get_vec2_in_direction(semicircle[semicircle.len() - 2], curve_a, 2.);
    let curve_c = vec2(curve_b.x, curve_b.y - curve_a.distance(curve_b));
    let quadratic_bezier: Vec<Vec2> = get_quadratic_bezier(curve_a, curve_b, curve_c);

    [semicircle[..semicircle.len() - 1], &quadratic_bezier].concat()
}

fn create_light_bulb_glass_mesh(num_rings: u16) -> Mesh {
    create_lathe_mesh(&create_light_bulb_glass_profile(), num_rings)
}

const CAP_SEGMENTS_TO_RINGS_RATIO: f32 = 1.;

fn create_light_bulb_cap_mesh(num_rings: u16) -> Mesh {}

pub fn create_light_bulb_mesh(num_rings: u16) -> Mesh {
    combine_meshes(&create_light_bulb_glass_mesh(num_rings), &create_light_bulb_cap_mesh(num_rings))
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
