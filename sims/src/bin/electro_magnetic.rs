use macroquad::prelude::*;
use sims::{camera_w_controls::CameraWControls, mesh::models::light_bulb::get_light_bulb_mesh};

mod cpp {
    use core::slice;
    use macroquad::math::{Vec2, vec2};

    mod bindings {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    pub fn get_baked_semicircle() -> &[Vec2] {
        unsafe {
            let ptr = bindings::get_baked_semicircle_ptr();
            let num_sections = bindings::get_baked_semicircle_num_sections();
            slice::from_raw_parts(ptr, num_sections as usize)
        }
    }
}

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let lightbulb = get_light_bulb_mesh();

    let mut camera = CameraWControls::default();
    camera.update(true);

    println!("Got {:?} from C++", cpp::get_baked_semicircle());

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
