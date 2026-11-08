use std::path::PathBuf;

use macroquad::conf::Conf;
use macroquad::prelude::*;
use sims::mesh::frag_feature::FragFeatures;
use sims::mesh::models::light_bulb::LightBulb;
use sims::three_d::camera_w_controls::CameraWControls;
use sims::util::macroquad::get_macroquad_conf;

fn window_conf() -> Conf {
    get_macroquad_conf("Electro-magnetic simulation")
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_base = PathBuf::from("./configs/electro_magnetic");

    let mut camera = CameraWControls::from_config(config_base.join("camera.toml").as_path())?;
    camera.update(true);

    let scene_frags = FragFeatures::from_config(config_base.join("scene.toml").as_path())?;

    let lightbulb = LightBulb::from_config(config_base.join("light_bulb.toml").as_path(), &scene_frags)?;
    // light_switch = create_light_switch_mesh(position);
    // wire_loop = create_wire_loop_mesh(positions);

    loop {
        lightbulb.draw();

        camera.update(false);

        next_frame().await;
        clear_background(BLACK);
    }
}
