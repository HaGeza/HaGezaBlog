use macroquad::{math::Vec3, ui::Vertex};

use crate::vec3_relative_eq;

/// Return `true` if `pos_a`, `pos_b`, `pos_c` define a (non-empty) triangle, `false` otherwise
fn is_non_empty_triangle(positions: &[Vec3; 3]) -> bool {
    !vec3_relative_eq!(positions[0], positions[1])
        && !vec3_relative_eq!(positions[0], positions[2])
        && !vec3_relative_eq!(positions[1], positions[2])
}

pub(super) fn add_face(vertices: &[Vertex], indices: &mut Vec<u16>, mut face_indices: [u16; 3], reverse_faces: bool) {
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

pub(super) struct RectangleFaceCorners {
    pub bot_left: u16,
    pub top_left: u16,
    pub bot_right: u16,
    pub top_right: u16,
}

pub(super) fn add_rectangle_face(
    vertices: &[Vertex],
    indices: &mut Vec<u16>,
    corners: RectangleFaceCorners,
    reverse_faces: bool,
) {
    // Add non-empty new faces in clockwise vertex order (for some reason, even though
    // the pipeline is supposed to use clounterclockwise order):
    add_face(vertices, indices, [corners.bot_right, corners.bot_left, corners.top_left], reverse_faces);
    add_face(vertices, indices, [corners.top_right, corners.bot_right, corners.top_left], reverse_faces);
}

#[cfg(test)]
mod tests {
    use macroquad::math::vec3;

    use super::*;

    #[test]
    fn test_is_non_empty_triangle_return_true_for_three_different_positions() {
        assert!(is_non_empty_triangle(&[vec3(1., 2., 3.), vec3(1., 2., 4.), vec3(0., 2., 4.)]));
    }

    #[test]
    fn test_is_non_empty_triangle_return_false_when_any_positions_are_equal() {
        assert!(!is_non_empty_triangle(&[vec3(1., 2., 3.), vec3(1., 2., 3.), vec3(0., 2., 4.)]));
        assert!(!is_non_empty_triangle(&[vec3(1., 2., 3.), vec3(1., 2., 4.), vec3(1., 2., 4.)]));
        assert!(!is_non_empty_triangle(&[vec3(1., 2., 3.), vec3(1., 2., 4.), vec3(1., 2., 3.)]));
        assert!(!is_non_empty_triangle(&[vec3(1., 2., 3.), vec3(1., 2., 3.), vec3(1., 2., 3.)]));
    }
}
