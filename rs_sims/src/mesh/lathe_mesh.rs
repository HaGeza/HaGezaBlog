//! Module for creating Lathe Meshes, see: https://en.wikipedia.org/wiki/Lathe_(graphics) for example

use super::common::is_non_empty_triangle;
use std::f32::consts::PI;

use macroquad::{
    color::WHITE,
    math::{Vec2, Vec3, Vec4},
    models::Mesh,
    ui::Vertex,
};

/** Return the vertices of the lathe mesh created  */
fn create_lathe_vertices(profile: &[Vec2], num_rings: u16) -> Vec<Vertex> {
    let mut vertices = vec![];
    for ring in 0..num_rings {
        for pt in profile {
            let theta: f32 = ring as f32 / num_rings as f32 * 2. * PI;
            let position = Vec3::new(pt.x * theta.cos(), pt.y, pt.x * theta.sin());
            vertices.push(Vertex {
                position: position,
                uv: Vec2::ZERO,
                color: WHITE.into(), // TODO
                normal: Vec4::ZERO,  // TODO
            });
        }
    }
    vertices
}

fn create_lathe_indices(profile: &[Vec2], num_rings: u16, vertices: &Vec<Vertex>) -> Vec<u16> {
    let mut indices = vec![];
    for ring in 0..num_rings {
        for pt_ind in 0..profile.len() - 1 {
            let top_right = ring as usize * profile.len() + pt_ind;
            let bot_right = ring as usize * profile.len() + pt_ind + 1;

            let next_ring = ((ring + 1) % num_rings) as usize;
            let top_left = next_ring * profile.len() + pt_ind;
            let bot_left = next_ring * profile.len() + pt_ind + 1;

            // Add non-empty new faces in counterclockwise vertex order:
            if is_non_empty_triangle(
                vertices[top_left].position,
                vertices[bot_left].position,
                vertices[bot_right].position,
            ) {
                indices.extend([top_left as u16, bot_left as u16, bot_right as u16]);
            }
            if is_non_empty_triangle(
                vertices[top_left].position,
                vertices[bot_right].position,
                vertices[top_right].position,
            ) {
                indices.extend([top_left as u16, bot_right as u16, top_right as u16]);
            }
        }
    }
    indices
}

/**
 * Create a mesh by rotating a 2D `profile` around the Y axis `num_rings` times.
 * The `profile` points are assumed to be clockwise ordered.
 */
pub fn create_lathe_mesh(profile: &[Vec2], num_rings: u16) -> Mesh {
    let vertices = create_lathe_vertices(profile, num_rings);
    let indices = create_lathe_indices(profile, num_rings, &vertices);
    Mesh {
        vertices: vertices,
        indices: indices,
        texture: None,
    }
}

#[cfg(test)]
mod tests {
    use macroquad::math::{vec2, vec3};

    use crate::assert_vec3_relative_eq;

    use super::*;

    #[test]
    fn test_create_lathe_vertices_creates_correct_number_of_vertices() {
        let profile = vec![vec2(1., 0.5), vec2(0., 1.), vec2(-1., 0.5)];

        assert_eq!(create_lathe_vertices(&profile, 2).len(), 6);
        assert_eq!(create_lathe_vertices(&profile, 3).len(), 9);
        assert_eq!(create_lathe_vertices(&profile, 4).len(), 12);
        assert_eq!(create_lathe_vertices(&profile, 20).len(), 60);
    }

    #[test]
    fn test_create_lathe_vertices_creates_vertices_in_correct_positions() {
        let profile = vec![vec2(0.5, 1.), vec2(1., 0.), vec2(0.5, -1.)];

        let vertices = create_lathe_vertices(&profile, 4);
        // ring 0
        assert_vec3_relative_eq!(vertices[0].position, vec3(0.5, 1., 0.));
        assert_vec3_relative_eq!(vertices[1].position, vec3(1., 0., 0.));
        assert_vec3_relative_eq!(vertices[2].position, vec3(0.5, -1., 0.));
        // ring 1
        assert_vec3_relative_eq!(vertices[3].position, vec3(0., 1., 0.5));
        assert_vec3_relative_eq!(vertices[4].position, vec3(0., 0., 1.));
        assert_vec3_relative_eq!(vertices[5].position, vec3(0., -1., 0.5));
        // ring 1
        assert_vec3_relative_eq!(vertices[6].position, vec3(-0.5, 1., 0.));
        assert_vec3_relative_eq!(vertices[7].position, vec3(-1., 0., 0.));
        assert_vec3_relative_eq!(vertices[8].position, vec3(-0.5, -1., 0.));
        // ring 1
        assert_vec3_relative_eq!(vertices[9].position, vec3(0., 1., -0.5));
        assert_vec3_relative_eq!(vertices[10].position, vec3(0., 0., -1.));
        assert_vec3_relative_eq!(vertices[11].position, vec3(0., -1., -0.5));
    }

    #[test]
    fn test_create_lathe_vertices_creates_no_vertices_with_zero_rings() {
        let profile = vec![vec2(0.5, 1.), vec2(0., 1.), vec2(0.5, -1.)];
        assert_eq!(create_lathe_vertices(&profile, 0).len(), 0);
    }

    #[test]
    fn test_create_lathe_indices_creates_faces_in_clockwise_order() {
        let profile = vec![vec2(0.5, 1.), vec2(0.5, -1.)];
        let vertices = vec![
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
        .map(|p| Vertex {
            position: *p,
            uv: Vec2::ZERO,
            color: WHITE.into(),
            normal: Vec4::ZERO,
        })
        .collect();

        let indices = create_lathe_indices(&profile, 4, &vertices);

        assert_eq!(indices.len(), 4 * 2 * 3); // 4 faces, 2 triangles each, 3 points each
        assert_eq!(
            indices,
            vec![
                2, 3, 1, // face 0
                2, 1, 0, // face 1
                4, 5, 3, // face 2
                4, 3, 2, // face 3
                6, 7, 5, // face 4
                6, 5, 4, // face 5
                0, 1, 7, // face 6
                0, 7, 6, // face 7
            ]
        );
    }

    #[test]
    fn test_create_lathe_indices_skips_empty_faces() {
        let profile = vec![vec2(0., 1.), vec2(1., 0.), vec2(0., -1.)];
        let vertices = vec![
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
        .map(|p| Vertex {
            position: *p,
            uv: Vec2::ZERO,
            color: WHITE.into(),
            normal: Vec4::ZERO,
        })
        .collect();

        let indices = create_lathe_indices(&profile, 4, &vertices);

        assert_eq!(indices.len(), 8 * 3); // 8 faces, 3 points each

        assert_eq!(
            indices,
            vec![
                3, 4, 1, // face 0
                4, 2, 1, // face 1
                6, 7, 4, // face 2
                7, 5, 4, // face 3
                9, 10, 7, // face 4
                10, 8, 7, // face 5
                0, 1, 10, // face 6
                1, 11, 10, // face 7
            ]
        );
    }

    #[test]
    fn test_create_lathe_mesh_creates_simple_mesh() {
        let profile = vec![vec2(1., 1.), vec2(1., -1.)];
        let mesh = create_lathe_mesh(&profile, 2);

        assert_eq!(mesh.vertices.len(), 4);
        assert_vec3_relative_eq!(mesh.vertices[0].position, vec3(1., 1., 0.));
        assert_vec3_relative_eq!(mesh.vertices[1].position, vec3(1., -1., 0.));
        assert_vec3_relative_eq!(mesh.vertices[2].position, vec3(-1., 1., 0.));
        assert_vec3_relative_eq!(mesh.vertices[3].position, vec3(-1., -1., 0.));

        assert_eq!(mesh.indices.len(), 2 * 2 * 3);
        assert_eq!(
            mesh.indices,
            vec![
                2, 3, 1, 2, 1, 0, //front
                0, 1, 3, 0, 3, 2 // back
            ]
        );
    }
}
