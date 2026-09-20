use macroquad::prelude::*;
use sims::{camera_w_controls::CameraWControls, mesh::models::light_bulb::get_light_bulb_mesh};
use std::slice;

unsafe extern "C" {
    fn get_baked_light_bulb_mesh_data() -> MeshData;
}

struct MeshData {
    vertex_count: usize,
    vertices: *const Vertex,
    index_count: usize,
    indices: *const u16,
}

fn get_baked_light_bulb_mesh() -> Mesh {
    unsafe {
        let mesh_data = get_baked_light_bulb_mesh_data();
        Mesh {
            vertices: slice::from_raw_parts(mesh_data.vertices, mesh_data.vertex_count).to_vec(),
            indices: slice::from_raw_parts(mesh_data.indices, mesh_data.index_count).to_vec(),
            texture: None,
        }
    }
}

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let lightbulb = get_baked_light_bulb_mesh();

    let mut camera = CameraWControls::default();
    camera.update(true);

    // light_switch = create_light_switch_mesh(position);
    // wire_loop = create_wire_loop_mesh(positions);

    loop {
        camera.update(false);
        // draw_cube(Vec3::ZERO, Vec3::ONE, None, RED);
        // draw_grid(20, 1.0, GRAY, DARKGRAY);
        draw_mesh(&lightbulb);

        next_frame().await;
        clear_background(BLACK);
    }
}
