use std::{
    f32::consts::PI,
    ops::{Mul, Sub},
};

use macroquad::math::{Vec2, vec2};

use crate::util::linalg::det_2d;

fn _into_circle(radian: f32) -> f32 {
    (radian + PI) % (2. * PI) - PI
}

pub fn get_semicircle(radius: f32, start_radian: f32, end_radian: f32, num_sections: usize) -> Vec<Vec2> {
    let start_radian = _into_circle(start_radian);
    let end_radian = _into_circle(end_radian);
    let end_radian = end_radian + if start_radian >= end_radian { 2. * PI } else { 0. };
    let ns = num_sections as f32;
    (0..=num_sections)
        .map(|i| (i as f32 / ns) * end_radian + ((num_sections - i) as f32 / ns) * start_radian)
        .map(|theta| vec2(theta.cos(), theta.sin()).mul(radius))
        .collect()
}

/**
 * Get intersection of two lines. Returns `None` if the lines are parallel.
 */
pub fn intersect(line_a: [&Vec2; 2], line_b: [&Vec2; 2]) -> Option<Vec2> {
    // We solve the system of linear equations:
    //   x * (a0.y - a1.y) + y (a1.x - a0.x) = -det(a)
    //   x * (b0.y - b1.y) + y (b1.x - b0.x) = -det(b)
    // Using Cramer's rule.

    let x_diffs = vec2(line_a[0].x - line_a[1].x, line_b[0].x - line_b[1].x);
    let y_diffs = vec2(line_a[0].y - line_a[1].y, line_b[0].y - line_b[1].y);
    let det_ab_transpose = det_2d([&x_diffs, &y_diffs]);

    if det_ab_transpose.abs() < f32::EPSILON {
        return None;
    }
    let line_dets = vec2(det_2d(line_a), det_2d(line_b));
    Some(vec2(det_2d([&line_dets, &x_diffs]) / det_ab_transpose, det_2d([&line_dets, &y_diffs]) / det_ab_transpose))
}

pub fn get_quadratic_bezier(pts: [&Vec2; 3], num_sections: usize) -> Vec<Vec2> {
    (0..=num_sections)
        .map(|i| i as f32 / num_sections as f32)
        .map(|t| (1. - t) * ((1. - t) * *pts[0] + t * *pts[1]) + t * ((1. - t) * *pts[1] + t * *pts[2]))
        .collect()
}

pub fn get_normal(pts: [&Vec2; 2]) -> Vec2 {
    let tangent = pts[1].sub(*pts[0]);
    vec2(tangent.y, -tangent.x).normalize()
}

#[cfg(test)]
mod tests {}
