use macroquad::conf::Conf;
use macroquad::prelude::*;
use sims::mesh::frag_feature::FragFeatures;
use sims::mesh::models::light_bulb::LightBulb;
use sims::mesh::models::light_switch::LightSwitch;
use sims::three_d::camera_w_controls::CameraWControls;
use sims::util::config::CONFIGS_BASE;
use sims::util::macroquad::get_macroquad_conf;

fn window_conf() -> Conf {
    get_macroquad_conf("Electro-magnetic simulation")
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_base = format!("{CONFIGS_BASE}/electro_magnetic");

    let mut camera = CameraWControls::from_config(format!("{config_base}/camera.toml").as_str()).await?;
    camera.update(true);

    let mut scene_frags = FragFeatures::from_config(format!("{config_base}/scene.toml").as_str()).await?;

    let lightbulb = LightBulb::from_config(format!("{config_base}/light_bulb.toml").as_str(), &scene_frags).await?;
    scene_frags.frags.push(*lightbulb.get_point_light());

    let light_switch =
        LightSwitch::from_config(format!("{config_base}/light_switch.toml").as_str(), &scene_frags).await?;
    // wire_loop = create_wire_loop_mesh(positions);

    loop {
        light_switch.draw();
        lightbulb.draw();

        camera.update(false);

        next_frame().await;
        clear_background(BLACK);
    }
}
