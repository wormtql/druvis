use cgmath::Vector4;

use crate::shader::shader::DruvisShader;

pub struct DruvisColorShaderProperties {
    pub color: Vector4<f32>,
}

pub struct DruvisColorShader<'a> {
    pub shader: DruvisShader<'a, DruvisColorShaderProperties>,
}

