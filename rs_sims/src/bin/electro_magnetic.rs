use macroquad::prelude::*;
use sims::camera_w_controls::CameraWControls;

#[macroquad::main("Electro-Magnetic Fields")]
async fn main() {
    let camera = CameraWControls::default();
    set_camera(&camera.camera);

    loop {
        draw_cube(Vec3::ZERO, Vec3::ONE, None, RED);

        next_frame().await;
    }
}
