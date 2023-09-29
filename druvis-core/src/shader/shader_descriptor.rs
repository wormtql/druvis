use wgpu::util::DeviceExt;
use serde::{Serialize, Deserialize};

use crate::{binding::bind_group_layout_builder::BindGroupLayoutBuilder, utils};

use super::{shader::OwnedVertexBufferLayout, shader_property::{ShaderPropertyLayoutEntry, ShaderTextureLayoutEntry, ShaderTexturePropertyType}};

#[derive(Serialize, Deserialize)]
pub struct ShaderDescriptor {
    pub name: String,
    pub source: String,
    pub cull_mode: Option<wgpu::Face>,
    pub blend_mode: Option<wgpu::BlendState>,
    pub is_instancing: bool,
    pub instancing_vertex_buffer_layout: Option<OwnedVertexBufferLayout>,
    pub shader_value_layout: Vec<ShaderPropertyLayoutEntry>,
    pub shader_texture_layout: Vec<ShaderTextureLayoutEntry>,
}

impl ShaderDescriptor {
    pub fn get_bind_group_layout(&self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let mut bind_group_layout_builder = BindGroupLayoutBuilder::new();
        
        bind_group_layout_builder.add_buffer_entry(device, 0);
        let mut binding = 1_u32;
        for item in self.shader_texture_layout.iter() {
            if item.ty == ShaderTexturePropertyType::Texture {
                bind_group_layout_builder.add_texture_entry(binding, item.texture_view_dimension.unwrap());
            } else if item.ty == ShaderTexturePropertyType::Sampler {
                bind_group_layout_builder.add_sampler_entry(binding, item.sampler_type.unwrap());
            }
            binding += 1;
        }

        bind_group_layout_builder.build(device, &self.name)
    }

    pub fn get_properties_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let mut size = 0;
        for item in self.shader_value_layout.iter() {
            size += item.ty.get_size();
        }

        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some((self.name.clone() + "_buffer").as_str()),
                contents: &utils::create_buffer(size),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        )
    }
}
