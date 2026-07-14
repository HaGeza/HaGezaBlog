use macroquad::math::Vec2;

pub fn det_2d(matrix: [&Vec2; 2]) -> f32 {
    matrix[0].x * matrix[1].y - matrix[0].y * matrix[1].x
}
