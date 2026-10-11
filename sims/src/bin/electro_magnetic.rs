use macroquad::conf::Conf;
use macroquad::prelude::*;
use sims::config::macroquad::get_macroquad_conf;
use sims::{camera_w_controls::CameraWControls, mesh::models::light_bulb::get_light_bulb};

fn window_conf() -> Conf {
    get_macroquad_conf("Electro-magnetic simulation")
}

#[macroquad::main(window_conf)]
async fn main() {
    let lightbulb = get_light_bulb();

    let mut camera = CameraWControls::default();
    camera.update(true);

    // light_switch = create_light_switch_mesh(position);
    // wire_loop = create_wire_loop_mesh(positions);

    loop {
        lightbulb.draw();

        camera.update(false);

        next_frame().await;
        clear_background(BLACK);
    }
}
