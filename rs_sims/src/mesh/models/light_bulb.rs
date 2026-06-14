use macroquad::models::Mesh;

use crate::mesh::common::combine_meshes;

const GLASS_SEGMENTS_TO_RINGS_RATIO: f32 = 1.;

fn create_light_bulb_glass_mesh(num_rings: u16) -> Mesh {
    let profile = create_light_bulb_glass_profile();
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
