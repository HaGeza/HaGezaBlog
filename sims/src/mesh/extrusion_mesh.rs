use earcut::Earcut;
use macroquad::{
    color::Color,
    math::{Vec2, Vec4},
    models::Mesh,
    ui::Vertex,
};

use crate::{mesh::plane::Plane, shape::profile::Profile};

fn create_vertices(profile: &Profile, plane: &Plane, thickness: f32, color: &Color) -> Vec<Vertex> {
    let offset = *plane.normal() * thickness;
    let positions = profile.iter_pts().flat_map(|pt| {
        let position = plane.project_2d(pt);
        [position, position + offset]
    });

    // TODO: calculate the normals of the faces at each vertex
    positions
        .flat_map(|position| {
            [
                Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal: Vec4::ZERO },
                Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal: Vec4::ZERO },
                Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal: Vec4::ZERO },
            ]
        })
        .collect()
}

fn create_indices(profile: &Profile) -> Vec<u16> {
    let mut profile_triangles = vec![];
    Earcut::new().earcut(
        profile.iter_pts().map(|v2| v2.to_array()).collect::<Vec<[f32; 2]>>(),
        &profile
            .holes()
            .iter()
            .scan(profile.outline().len(), |acc, hole| {
                *acc += hole.len();
                Some(*acc - hole.len())
            })
            .collect::<Vec<usize>>(),
        &mut profile_triangles,
    );

    // TODO: add faces on
    // - the previous side (first normal for each position)
    // - the next side (second normal for each position)
    // - the triangulated surface (last normal for each position)
    //
    // add_side_indices()
    // add_top_and_bottom_indices()
    vec![]
}

pub fn extrude_profile(profile: &Profile, plane: &Plane, thickness: f32, color: &Color) -> Mesh {
    let vertices = create_vertices(profile, plane, thickness, color);
    let indices = create_indices(profile);
    Mesh { vertices, indices, texture: None }
}
