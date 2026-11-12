use earcut::Earcut;
use macroquad::{
    color::Color,
    math::{Vec2, Vec4},
    models::Mesh,
    ui::Vertex,
};

use crate::{
    mesh::{
        common::{RectangleFaceCorners, add_face, add_rectangle_face},
        plane::Plane,
    },
    shape::profile::Profile,
};

enum VertexSide {
    Previous,
    Next,
    Base,
}

fn get_mesh_ind(profile_ind: usize, is_extrusion: bool, vertex_side: VertexSide) -> u16 {
    (profile_ind * 6) as u16
        + (is_extrusion as u16) * 3
        + match vertex_side {
            VertexSide::Previous => 0,
            VertexSide::Next => 1,
            VertexSide::Base => 2,
        }
}

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
                Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal: Vec4::ZERO }, // previous side
                Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal: Vec4::ZERO }, // next side
                Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal: Vec4::ZERO }, // top/bottom
            ]
        })
        .collect()
}

fn add_profile_element_side_indices(
    element: &[Vec2],
    offset: usize,
    vertices: &[Vertex],
    indices: &mut Vec<u16>,
    reverse_faces: bool,
) {
    for edge in 0..element.len() {
        let next_edge = (edge + 1) % element.len();
        let corners = RectangleFaceCorners {
            bot_left: get_mesh_ind(offset + edge, false, VertexSide::Next),
            top_left: get_mesh_ind(offset + edge, true, VertexSide::Next),
            bot_right: get_mesh_ind(offset + next_edge, false, VertexSide::Previous),
            top_right: get_mesh_ind(offset + next_edge, true, VertexSide::Previous),
        };
        add_rectangle_face(vertices, indices, corners, reverse_faces);
    }
}

fn add_side_indices(profile: &Profile, vertices: &[Vertex], indices: &mut Vec<u16>) {
    let mut offset = 0;
    add_profile_element_side_indices(profile.outline(), offset, vertices, indices, false);

    offset += profile.outline().len();
    for hole in profile.holes() {
        add_profile_element_side_indices(hole, offset, vertices, indices, true);
        offset += hole.len()
    }
}

fn add_base_indices(profile: &Profile, vertices: &[Vertex], indices: &mut Vec<u16>) {
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

    for triangle in profile_triangles.chunks(3) {
        let (a, b, c) = (profile[triangle[0]], profile[triangle[1]], profile[triangle[2]]);
        let is_counterclockwise = (b - a).perp_dot(c - a) > 0.0;
        let triangle = [triangle[0], triangle[1], triangle[2]];
        add_face(vertices, indices, triangle.map(|i| get_mesh_ind(i, false, VertexSide::Base)), !is_counterclockwise);
        add_face(vertices, indices, triangle.map(|i| get_mesh_ind(i, true, VertexSide::Base)), is_counterclockwise);
    }
}

fn create_indices(profile: &Profile, vertices: &[Vertex]) -> Vec<u16> {
    let mut indices = vec![];
    add_side_indices(profile, vertices, &mut indices);
    add_base_indices(profile, vertices, &mut indices);
    indices
}

pub fn extrude_profile(profile: &Profile, plane: &Plane, thickness: f32, color: &Color) -> Mesh {
    let vertices = create_vertices(profile, plane, thickness, color);
    let indices = create_indices(profile, &vertices);
    Mesh { vertices, indices, texture: None }
}
