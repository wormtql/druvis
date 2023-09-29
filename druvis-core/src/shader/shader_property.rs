use cgmath::{Vector4, Matrix4};
use serde::{Serialize, Deserialize};

use crate::common::util_traits::AsBytes;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ShaderPropertyType {
    Vec4,
    Mat4,
    U32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ShaderTexturePropertyType {
    Texture,
    Sampler,
}

impl ShaderPropertyType {
    pub fn get_size(&self) -> usize {
        match *self {
            ShaderPropertyType::Mat4 => 16 * 4,
            ShaderPropertyType::Vec4 => 4 * 4,
            ShaderPropertyType::U32 => 4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShaderPropertyValue {
    Vec4(Vector4<f32>),
    Mat4(Matrix4<f32>),
    U32(u32),
}

impl ShaderPropertyValue {
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            ShaderPropertyValue::Vec4(x) => x.druvis_as_bytes(),
            ShaderPropertyValue::Mat4(x) => x.druvis_as_bytes(),
            ShaderPropertyValue::U32(x) => x.druvis_as_bytes(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShaderPropertyLayoutEntry {
    pub ty: ShaderPropertyType,
    pub name: String,
    pub default_value: ShaderPropertyValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShaderTextureLayoutEntry {
    pub ty: ShaderTexturePropertyType,
    pub name: String,
    pub texture_view_dimension: Option<wgpu::TextureViewDimension>,
    pub sampler_type: Option<wgpu::SamplerBindingType>,
}
