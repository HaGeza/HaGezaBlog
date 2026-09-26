#[macro_export]
macro_rules! load_shader_material {
    ($name:literal) => {
        load_material(
            miniquad::ShaderSource::Glsl {
                vertex: include_str!(concat!("../shaders/versions/", env!("GLSL_VERSION"), "/", $name, ".vert")),
                fragment: include_str!(concat!("../shaders/versions/", env!("GLSL_VERSION"), "/", $name, ".frag")),
            },
            MaterialParams::default(),
        )
    };
}
