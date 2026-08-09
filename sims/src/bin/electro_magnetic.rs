use macroquad::prelude::*;
use sims::{camera_w_controls::CameraWControls, mesh::models::light_bulb::get_light_bulb_mesh};

mod cpp {
    mod bindings {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    pub fn tmp() -> i32 {
        unsafe { bindings::tmp_fn() }
    }
}

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let lightbulb = get_light_bulb_mesh();

    let mut camera = CameraWControls::default();
    camera.update(true);

    println!("Got {} from C++", cpp::tmp());

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
