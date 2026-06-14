use macroquad::prelude::*;
use sims::camera_w_controls::CameraWControls;
use sims::mesh::lathe_mesh::create_lathe_mesh;

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let mut camera = CameraWControls::default();
    camera.update(true);

    let lathe_mesh = create_lathe_mesh();
    // lightbulb = create_lightbulb_mesh(position);
    // light_switch = create_light_switch_mesh(position);
    // wire_loop = create_wire_loop_mesh(positions);

    loop {
        camera.update(false);
        draw_cube(Vec3::ZERO, Vec3::ONE, None, RED);
        draw_grid(20, 1.0, GRAY, DARKGRAY);

        next_frame().await;
        clear_background(BLACK);
    }
}
