#[macro_export]
macro_rules! load_shader_material {
    ($vert:literal, $frag:literal) => {
        load_material(
            miniquad::ShaderSource::Glsl {
                vertex: include_str!(concat!("../shaders/versions/", env!("GLSL_VERSION"), "/", $vert, ".vert")),
                fragment: include_str!(concat!("../shaders/versions/", env!("GLSL_VERSION"), "/", $frag, ".frag")),
            },
            MaterialParams {
                uniforms: vec![UniformDesc::new("light_direction", UniformType::Float3)],
                pipeline_params: PipelineParams {
                    depth_test: Comparison::LessOrEqual,
                    depth_write: true,
                    ..Default::default()
                },
                ..MaterialParams::default()
            },
        )
    };
}
