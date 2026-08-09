use macroquad::{math::Vec3, models::Mesh};

use crate::vec3_relative_eq;

/// Return `true` if `pos_a`, `pos_b`, `pos_c` define a (non-empty) triangle, `false` otherwise
pub(super) fn is_non_empty_triangle(pos_a: Vec3, pos_b: Vec3, pos_c: Vec3) -> bool {
    !vec3_relative_eq!(pos_a, pos_b) && !vec3_relative_eq!(pos_a, pos_c) && !vec3_relative_eq!(pos_b, pos_c)
}

#[allow(dead_code)]
fn combine_meshes(mesh_a: &Mesh, mesh_b: &Mesh) -> Mesh {
    Mesh {
        vertices: mesh_a.vertices.iter().chain(mesh_b.vertices.iter()).cloned().collect(),
        indices: mesh_a
            .indices
            .iter()
            .cloned()
            .chain(mesh_b.indices.iter().map(|i| i + mesh_a.vertices.len() as u16))
            .collect(),
        texture: None,
    }
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec3;

    use super::*;

    #[test]
    fn test_is_non_empty_triangle_return_true_for_three_different_positions() {
        assert!(is_non_empty_triangle(vec3(1., 2., 3.), vec3(1., 2., 4.), vec3(0., 2., 4.)));
    }

    #[test]
    fn test_is_non_empty_triangle_return_false_when_any_positions_are_equal() {
        assert!(!is_non_empty_triangle(vec3(1., 2., 3.), vec3(1., 2., 3.), vec3(0., 2., 4.)));
        assert!(!is_non_empty_triangle(vec3(1., 2., 3.), vec3(1., 2., 4.), vec3(1., 2., 4.)));
        assert!(!is_non_empty_triangle(vec3(1., 2., 3.), vec3(1., 2., 4.), vec3(1., 2., 3.)));
        assert!(!is_non_empty_triangle(vec3(1., 2., 3.), vec3(1., 2., 3.), vec3(1., 2., 3.)));
    }

    #[test]
    fn test_combine_meshes_combines_vertices() {
        assert!(false);
    }

    #[test]
    fn test_combine_meshes_combines_updates_and_combines() {
        assert!(false);
    }
}
