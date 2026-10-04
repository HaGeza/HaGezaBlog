use macroquad::Error;
use macroquad::material::{Material, MaterialParams, load_material};
use macroquad::math::Vec3;
use miniquad::{Comparison, PipelineParams, UniformDesc, UniformType};

macro_rules! load_shader {
    ($file:literal, $type:literal) => {
        include_str!(concat!("../shaders/versions/", env!("GLSL_VERSION"), "/", $file, ".", $type))
    };
}

macro_rules! load_vert_shader {
    ($file:literal) => {
        load_shader!($file, "vert")
    };
}

macro_rules! load_frag_shader {
    ($file:literal) => {
        load_shader!($file, "frag")
    };
}

pub enum VertShader {
    General,
}

pub enum FragShader {
    DirectionalLight(Vec3),
}

pub fn load_shader_material(vert: VertShader, frag: FragShader) -> Result<Material, Error> {
    let mut uniforms: Vec<UniformDesc> = vec![];

    let vert_shader = match vert {
        VertShader::General => load_vert_shader!("general"),
    };
    let frag_shader = match frag {
        FragShader::DirectionalLight(_) => {
            uniforms.push(UniformDesc::new("light_direction", UniformType::Float3));
            load_frag_shader!("directional_light")
        },
    };
    let shader_source = miniquad::ShaderSource::Glsl { vertex: vert_shader, fragment: frag_shader };

    let material_params = MaterialParams {
        uniforms,
        pipeline_params: PipelineParams {
            depth_test: Comparison::LessOrEqual,
            depth_write: true,
            ..Default::default()
        },
        ..MaterialParams::default()
    };

    let material = load_material(shader_source, material_params)?;
    match frag {
        FragShader::DirectionalLight(light_direction) => {
            material.set_uniform("light_direction", light_direction);
        },
    }
    Ok(material)
}
