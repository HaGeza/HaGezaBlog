use earcut::Earcut;
use macroquad::models::Mesh;

use crate::{mesh::plane::Plane, shape::profile::Profile};

pub fn extrude_profile(profile: &Profile, plane: &Plane, thickness: f32) -> Mesh {
    let mut profile_triangles = vec![];
    Earcut::new().earcut(
        profile
            .outline()
            .into_iter()
            .chain(profile.holes().iter().flatten())
            .map(|v2| v2.to_array())
            .collect::<Vec<[f32; 2]>>(),
        &profile
            .holes()
            .iter()
            .scan(profile.outline().len(), |acc, hole| {
                *acc = *acc + hole.len();
                Some(*acc - hole.len())
            })
            .collect::<Vec<usize>>(),
        &mut profile_triangles,
    );

    Mesh { vertices: vec![], indices: vec![], texture: None }
}
