use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use macroquad::material::{Material, MaterialParams, load_material};
use macroquad::math::Vec3;
use miniquad::{
    BlendFactor, BlendState, BlendValue, Comparison, CullFace, Equation, PipelineParams, UniformDesc, UniformType,
};
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
const SHADER_HEADER: &str = "#version 300 es\n";

#[cfg(not(target_arch = "wasm32"))]
const SHADER_HEADER: &str = "#version 330\n";

trait GlslOutputVar {
    fn declaration_str(&self) -> String;

    fn output_str(&self) -> String {
        format!("out {};", self.declaration_str())
    }

    fn input_str(&self) -> String {
        format!("in {};", self.declaration_str())
    }
}

#[derive(Eq, PartialEq, Hash)]
enum VertOutput {
    Color,
    Normal,
}

impl GlslOutputVar for VertOutput {
    fn declaration_str(&self) -> String {
        match self {
            Self::Color => "vec4 color",
            Self::Normal => "vec3 normal3",
        }
        .to_string()
    }
}

impl VertOutput {
    fn definition_str(&self) -> String {
        match self {
            Self::Color => "color = color0;",
            Self::Normal => "normal3 = normalize(transpose(inverse(mat3(Model))) * normal.xyz);",
        }
        .to_string()
    }
}

#[derive(Eq, PartialEq, Hash)]
enum FragOutput {
    FinalColor,
}

impl GlslOutputVar for FragOutput {
    fn declaration_str(&self) -> String {
        match self {
            Self::FinalColor => "vec4 final_color",
        }
        .to_string()
    }
}

const LIGHTING_VAR: &str = "lighting";

impl FragOutput {
    fn init_str(&self) -> String {
        match self {
            Self::FinalColor => format!("vec3 {LIGHTING_VAR} = vec3(0.0);"),
        }
        .to_string()
    }

    fn definition_str(&self) -> String {
        match self {
            Self::FinalColor => format!("final_color = vec4(color.rgb * {LIGHTING_VAR}, color.a) / 255.0;"),
        }
        .to_string()
    }
}

pub enum FragFeature {
    DirectionalLight(Vec3),
    AmbientLight(f32),
}

enum UniformWithValue {
    Float3(Vec3),
    Float(f32),
}

impl UniformWithValue {
    fn declaration_str(&self, name: &str) -> String {
        format!(
            "uniform {} {};",
            match self {
                Self::Float3(_) => "vec3",
                Self::Float(_) => "float",
            },
            name
        )
    }
}

impl From<&UniformWithValue> for UniformType {
    fn from(uniform_with_value: &UniformWithValue) -> UniformType {
        match uniform_with_value {
            UniformWithValue::Float3(_) => UniformType::Float3,
            UniformWithValue::Float(_) => UniformType::Float1,
        }
    }
}

#[derive(Default)]
pub struct MaterialBuilder {
    vert_outputs: HashSet<VertOutput>,
    frag_outputs: HashSet<FragOutput>,
    frag_uniforms: HashMap<String, UniformWithValue>,
    frag_feature_uses: Vec<String>,
    transparency: bool,
}

#[derive(Error, Debug)]
pub enum MaterialBuilderError {
    #[error("macroquad failure during material build: {0}")]
    MacroquadError(#[from] macroquad::Error),
}

impl MaterialBuilder {
    pub fn new() -> Self {
        MaterialBuilder::default()
    }

    pub fn build(&self) -> Result<Material, MaterialBuilderError> {
        let material = load_material(
            miniquad::ShaderSource::Glsl { vertex: &self.build_vert_shader(), fragment: &self.build_frag_shader() },
            self.build_material_params(),
        )?;
        for (name, val) in &self.frag_uniforms {
            match val {
                UniformWithValue::Float3(f3) => {
                    material.set_uniform(name, *f3);
                },
                UniformWithValue::Float(f) => {
                    material.set_uniform(name, *f);
                },
            };
        }
        Ok(material)
    }

    pub fn add_frag_features(&mut self, frag_features: &[FragFeature]) -> Result<&Self, MaterialBuilderError> {
        for frag_feature in frag_features {
            self.add_frag_feature(frag_feature)?
        }
        Ok(self)
    }

    pub fn set_transparency(&mut self, transparency: bool) -> &Self {
        self.transparency = transparency;
        self
    }

    fn add_frag_feature(&mut self, frag_feature: &FragFeature) -> Result<(), MaterialBuilderError> {
        match frag_feature {
            FragFeature::DirectionalLight(light_direction) => {
                self.vert_outputs.extend([VertOutput::Color, VertOutput::Normal]);
                self.frag_outputs.insert(FragOutput::FinalColor);

                let uniform_name = self.get_next_uniform_name("light_direction");
                self.frag_feature_uses
                    .push(format!("{LIGHTING_VAR} += max(dot(normalize(normal3), normalize({uniform_name})), 0.0);"));
                self.frag_uniforms.insert(uniform_name, UniformWithValue::Float3(*light_direction));
            },
            FragFeature::AmbientLight(ambient_intensity) => {
                self.vert_outputs.insert(VertOutput::Color);
                self.frag_outputs.insert(FragOutput::FinalColor);

                let uniform_name = self.get_next_uniform_name("ambient_intensity");
                self.frag_feature_uses.push(format!("{LIGHTING_VAR} += {uniform_name};"));
                self.frag_uniforms.insert(uniform_name, UniformWithValue::Float(*ambient_intensity));
            },
        };
        Ok(())
    }

    fn get_next_uniform_name(&self, name: &str) -> String {
        let mut counter = 0;
        loop {
            let name = format!("{}{}", name, counter);
            if !self.frag_uniforms.contains_key(&name) {
                return name;
            }
            counter += 1;
        }
    }

    fn build_vert_shader(&self) -> String {
        format!(
            "{}
precision highp float;

in vec3 position;
in vec4 color0;
in vec4 normal;

uniform mat4 Model;
uniform mat4 Projection;

{}

void main() {{
    gl_Position = Projection * Model * vec4(position, 1.0);
    {}
}}",
            SHADER_HEADER,
            self.vert_outputs.iter().map(|vo| vo.output_str()).join("\n"),
            self.vert_outputs.iter().map(|vo| vo.definition_str()).join("\n    "),
        )
    }

    fn build_frag_shader(&self) -> String {
        format!(
            "{}
precision highp float;

{}

{}

{}

void main() {{
    {}

    {}

    {}
}}",
            SHADER_HEADER,
            self.vert_outputs.iter().map(|vo| vo.input_str()).join("\n"),
            self.frag_uniforms.iter().map(|(name, val)| val.declaration_str(name)).join("\n"),
            self.frag_outputs.iter().map(|fo| fo.output_str()).join("\n"),
            self.frag_outputs.iter().map(|fo| fo.init_str()).join("\n    "),
            self.frag_feature_uses.iter().join("\n    "),
            self.frag_outputs.iter().map(|fo| fo.definition_str()).join("\n    "),
        )
    }

    // TODO
    fn build_material_params(&self) -> MaterialParams {
        let mut pipeline_params =
            PipelineParams { cull_face: CullFace::Back, depth_test: Comparison::LessOrEqual, ..Default::default() };

        if self.transparency {
            pipeline_params.depth_write = false;
            pipeline_params.color_blend = Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            ));
        } else {
            pipeline_params.depth_write = true;
        }

        MaterialParams {
            uniforms: self.frag_uniforms.iter().map(|(name, val)| UniformDesc::new(name, val.into())).collect(),
            pipeline_params,
            ..MaterialParams::default()
        }
    }
}
