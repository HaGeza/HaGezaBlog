//! Module for creating Lathe Meshes, see: https://en.wikipedia.org/wiki/Lathe_(graphics) for example

use crate::shape::lathe_profile::LatheProfile;

use super::common::is_non_empty_triangle;
use std::f32::consts::PI;

use macroquad::{
    color::Color,
    math::{Vec2, Vec3, Vec4},
    models::Mesh,
    ui::Vertex,
};

/** Return the vertices of the lathe mesh created  */
fn create_lathe_vertices(profile: &LatheProfile, num_rings: usize, color: &Color) -> Vec<Vertex> {
    let mut vertices = vec![];

    for ring in 0..num_rings {
        for (pt, normal) in profile.points().iter().zip(profile.normals().iter()) {
            let theta: f32 = ring as f32 / num_rings as f32 * 2. * PI;
            let position = Vec3::new(pt.x * theta.cos(), pt.y, pt.x * theta.sin());
            let normal = Vec4::new(normal.x * theta.cos(), normal.y, normal.x * theta.sin(), 0.);
            vertices.push(Vertex { position, uv: Vec2::ZERO, color: (*color).into(), normal });
        }
    }
    vertices
}

fn add_face(vertices: &[Vertex], indices: &mut Vec<u16>, mut face_indices: [u16; 3], reverse_faces: bool) {
    if is_non_empty_triangle(&[
        vertices[face_indices[0] as usize].position,
        vertices[face_indices[1] as usize].position,
        vertices[face_indices[2] as usize].position,
    ]) {
        if reverse_faces {
            face_indices.reverse();
        }
        indices.extend(face_indices);
    }
}

fn create_lathe_indices(
    profile: &LatheProfile,
    num_rings: usize,
    vertices: &[Vertex],
    reverse_faces: bool,
) -> Vec<u16> {
    let mut indices = vec![];
    for ring in 0..num_rings {
        for pt_ind in 0..profile.len() - 1 {
            let bot_left = ring * profile.len() + pt_ind;
            let top_left = ring * profile.len() + pt_ind + 1;

            let next_ring = (ring + 1) % num_rings;
            let bot_right = next_ring * profile.len() + pt_ind;
            let top_right = next_ring * profile.len() + pt_ind + 1;

            // Add non-empty new faces in counterclockwise vertex order:
            add_face(vertices, &mut indices, [top_left as u16, bot_left as u16, bot_right as u16], reverse_faces);
            add_face(vertices, &mut indices, [top_left as u16, bot_right as u16, top_right as u16], reverse_faces);
        }
    }
    indices
}

/**
 * Create a mesh by rotating a 2D `profile` around the Y axis `num_rings` times.
 * The `profile` points are assumed to be counterclockwise ordered.
 */
pub fn create_lathe_mesh(profile: &LatheProfile, num_rings: usize, color: &Color, reverse_faces: bool) -> Mesh {
    let vertices = create_lathe_vertices(profile, num_rings, color);
    let indices = create_lathe_indices(profile, num_rings, &vertices, reverse_faces);
    Mesh { vertices, indices, texture: None }
}

#[cfg(test)]
mod tests {
    use macroquad::color;
    use macroquad::math::{vec2, vec3};

    use crate::assert_vec3_relative_eq;

    use super::*;

    #[test]
    fn test_create_lathe_vertices_creates_correct_number_of_vertices() {
        let profile = LatheProfile::new(&[vec2(1., 1.), vec2(0., 0.5), vec2(0.5, 0.)]).unwrap();

        assert_eq!(create_lathe_vertices(&profile, 2, &color::WHITE).len(), 6);
        assert_eq!(create_lathe_vertices(&profile, 3, &color::WHITE).len(), 9);
        assert_eq!(create_lathe_vertices(&profile, 4, &color::WHITE).len(), 12);
        assert_eq!(create_lathe_vertices(&profile, 20, &color::WHITE).len(), 60);
    }

    #[test]
    fn test_create_lathe_vertices_creates_vertices_in_correct_positions() {
        let profile = LatheProfile::new(&[vec2(1., 1.), vec2(0., 0.5), vec2(0.5, 0.)]).unwrap();

        let vertices = create_lathe_vertices(&profile, 4, &color::WHITE);
        // ring 0
        assert_vec3_relative_eq!(vertices[0].position, vec3(1., 1., 0.));
        assert_vec3_relative_eq!(vertices[1].position, vec3(0., 0.5, 0.));
        assert_vec3_relative_eq!(vertices[2].position, vec3(0.5, 0., 0.));
        // ring 1
        assert_vec3_relative_eq!(vertices[3].position, vec3(0., 1., 1.));
        assert_vec3_relative_eq!(vertices[4].position, vec3(0., 0.5, 0.));
        assert_vec3_relative_eq!(vertices[5].position, vec3(0., 0., 0.5));
        // ring 1
        assert_vec3_relative_eq!(vertices[6].position, vec3(-1., 1., 0.));
        assert_vec3_relative_eq!(vertices[7].position, vec3(0., 0.5, 0.));
        assert_vec3_relative_eq!(vertices[8].position, vec3(-0.5, 0., 0.));
        // ring 1
        assert_vec3_relative_eq!(vertices[9].position, vec3(0., 1., -1.));
        assert_vec3_relative_eq!(vertices[10].position, vec3(0., 0.5, 0.));
        assert_vec3_relative_eq!(vertices[11].position, vec3(0., 0., -0.5));
    }

    #[test]
    fn test_create_lathe_vertices_creates_no_vertices_with_zero_rings() {
        let profile = LatheProfile::new(&[vec2(1., 1.), vec2(0., 0.5), vec2(0.5, 0.)]).unwrap();
        assert_eq!(create_lathe_vertices(&profile, 0, &color::WHITE).len(), 0);
    }

    #[test]
    fn test_create_lathe_indices_creates_faces_in_counterclockwise_order() {
        let profile = LatheProfile::new(&[vec2(0.5, 1.), vec2(0.5, -1.)]).unwrap();
        let vertices: Vec<Vertex> = [
            vec3(0.5, 1., 0.),
            vec3(0.5, -1., 0.),
            vec3(0., 1., 0.5),
            vec3(0., -1., 0.5),
            vec3(-0.5, 1., 0.),
            vec3(-0.5, -1., 0.),
            vec3(0., 1., -0.5),
            vec3(0., -1., -0.5),
        ]
        .iter()
        .map(|p| Vertex { position: *p, uv: Vec2::ZERO, color: color::WHITE.into(), normal: Vec4::ZERO })
        .collect();

        let indices = create_lathe_indices(&profile, 4, &vertices, false);

        assert_eq!(indices.len(), 4 * 2 * 3); // 4 faces, 2 triangles each, 3 points each
        assert_eq!(
            indices,
            vec![
                1, 0, 2, // face 0
                1, 2, 3, // face 1
                3, 2, 4, // face 2
                3, 4, 5, // face 3
                5, 4, 6, // face 4
                5, 6, 7, // face 5
                7, 6, 0, // face 6
                7, 0, 1, // face 7
            ]
        );
    }

    #[test]
    fn test_create_lathe_indices_skips_empty_faces() {
        let profile = LatheProfile::new(&[vec2(0., 1.), vec2(1., 0.), vec2(0., -1.)]).unwrap();
        let vertices: Vec<Vertex> = [
            vec3(0., 1., 0.), // ring 0
            vec3(1., 0., 0.),
            vec3(0., -1., 0.),
            vec3(0., 1., 0.), // ring 1
            vec3(0., 0., 1.),
            vec3(0., -1., 0.),
            vec3(0., 1., 0.), // ring 2
            vec3(-1., 0., 0.),
            vec3(0., -1., 0.),
            vec3(0., 1., 0.), // ring 3
            vec3(0., 0., -1.),
            vec3(0., -1., 0.),
        ]
        .iter()
        .map(|p| Vertex { position: *p, uv: Vec2::ZERO, color: color::WHITE.into(), normal: Vec4::ZERO })
        .collect();

        let indices = create_lathe_indices(&profile, 4, &vertices, false);

        assert_eq!(indices.len(), 8 * 3); // 8 faces, 3 points each

        assert_eq!(
            indices,
            vec![
                1, 3, 4, // face 0
                2, 1, 4, // face 1
                4, 6, 7, // face 2
                5, 4, 7, // face 3
                7, 9, 10, // face 4
                8, 7, 10, // face 5
                10, 0, 1, // face 6
                11, 10, 1 // face 7
            ]
        );
    }

    #[test]
    fn test_create_lathe_mesh_creates_simple_mesh() {
        let profile = LatheProfile::new(&[vec2(1., 1.), vec2(1., -1.)]).unwrap();
        let mesh = create_lathe_mesh(&profile, 2, &color::WHITE, false);

        assert_eq!(mesh.vertices.len(), 4);
        assert_vec3_relative_eq!(mesh.vertices[0].position, vec3(1., 1., 0.));
        assert_vec3_relative_eq!(mesh.vertices[1].position, vec3(1., -1., 0.));
        assert_vec3_relative_eq!(mesh.vertices[2].position, vec3(-1., 1., 0.));
        assert_vec3_relative_eq!(mesh.vertices[3].position, vec3(-1., -1., 0.));

        assert_eq!(mesh.indices.len(), 2 * 2 * 3);
        assert_eq!(
            mesh.indices,
            vec![
                1, 0, 2, 1, 2, 3, // front
                3, 2, 0, 3, 0, 1, // back
            ]
        );
    }
}
