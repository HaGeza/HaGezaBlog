use macroquad::prelude::*;
use sims::camera_w_controls::CameraWControls;

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let mut camera = CameraWControls::default();
    camera.update(true);

    loop {
        camera.update(false);
        draw_cube(Vec3::ZERO, Vec3::ONE, None, RED);
        draw_grid(20, 1.0, GRAY, DARKGRAY);

        next_frame().await;
        clear_background(BLACK);
    }
}
