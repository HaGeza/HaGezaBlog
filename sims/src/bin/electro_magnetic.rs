use macroquad::prelude::*;
use sims::{camera_w_controls::CameraWControls, mesh::models::light_bulb::get_light_bulb_mesh};
use std::slice;

unsafe extern "C" {
    fn get_baked_semicircle_ptr() -> *const Vec2;
    fn get_baked_semicircle_num_sections() -> u32;
}

fn get_baked_semicircle() -> &'static [Vec2] {
    unsafe {
        let ptr = get_baked_semicircle_ptr();
        let num_sections = get_baked_semicircle_num_sections();
        slice::from_raw_parts(ptr, num_sections as usize)
    }
}

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let lightbulb = get_light_bulb_mesh();

    let mut camera = CameraWControls::default();
    camera.update(true);

    println!("Got {:?} from C++", get_baked_semicircle());

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
