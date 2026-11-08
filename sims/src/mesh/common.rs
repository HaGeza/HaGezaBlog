use macroquad::math::Vec3;

use crate::vec3_relative_eq;

/// Return `true` if `pos_a`, `pos_b`, `pos_c` define a (non-empty) triangle, `false` otherwise
pub(super) fn is_non_empty_triangle(positions: &[Vec3; 3]) -> bool {
    !vec3_relative_eq!(positions[0], positions[1])
        && !vec3_relative_eq!(positions[0], positions[2])
        && !vec3_relative_eq!(positions[1], positions[2])
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
