use macroquad::conf::Conf;
use macroquad::prelude::*;
use sims::util::macroquad::get_macroquad_conf;
use sims::{camera_w_controls::CameraWControls, mesh::models::light_bulb::get_light_bulb};

fn window_conf() -> Conf {
    get_macroquad_conf("Electro-magnetic simulation")
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lightbulb = get_light_bulb(Mat4::from_translation(vec3(0., 0., -2.)));

    let mut camera = CameraWControls::from_config("./configs/electro_magnetic/camera.toml").unwrap();
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
