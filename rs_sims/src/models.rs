use std::f32::consts::PI;

use macroquad::{
    math::{Vec2, Vec3, Vec4},
    models::Mesh,
    ui::Vertex,
};

/**
 * Create a mesh by rotating a 2D `profile` around the Y axis `num_rings` times.
 * The `profile` points are assumed to be ordered along the Y axis top to bottom
 *
 * See: https://en.wikipedia.org/wiki/Lathe_(graphics) for example
 */
fn create_lathe_mesh(profile: &[Vec2], num_rings: u32) -> Mesh {
    let mut vertices = vec![];
    for ring in 0..num_rings {
        for pt in profile {
            let theta: f32 = ring as f32 / num_rings as f32 * 2. * PI;
            let position = Vec3::new(pt.x * theta.cos(), pt.y, pt.x * theta.sin());
            vertices.push(Vertex {
                position: position,
                uv: Vec2::ZERO,
                color: [255, 255, 255, 255], // TODO
                normal: Vec4::ZERO,          // TODO
            });
        }
    }

    let mut indices = vec![];
    for ring in 0..num_rings - 1 {
        for pt_ind in 0..profile.len() - 1 {
            let top_left = ring as u16 * profile.len() as u16 + pt_ind as u16;
            let bot_left = ring as u16 * profile.len() as u16 + pt_ind as u16 + 1;
            let top_right = (ring + 1) as u16 * profile.len() as u16 + pt_ind as u16;
            let bot_right = (ring + 1) as u16 * profile.len() as u16 + pt_ind as u16 + 1;

            // Add new faces in counterclockwise vertex order:
            indices.extend([
                top_left, bot_left, bot_right, top_left, bot_right, top_right,
            ]);
        }
    }

    Mesh {
        vertices: vertices,
        indices: indices,
        texture: None,
    }
}

pub fn create_lightbulb_mesh(position: &Vec3) -> Mesh {}

pub fn create_light_switch_mesh(position: &Vec3) -> Mesh {}

pub fn create_wire_loop_mesh(positions: &[Vec3]) -> Mesh {}
