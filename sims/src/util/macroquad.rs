use macroquad::conf::Conf;

pub fn get_macroquad_conf(title: &str) -> Conf {
    Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: title.to_owned(),
            platform: miniquad::conf::Platform {
                webgl_version: miniquad::conf::WebGLVersion::WebGL2,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
