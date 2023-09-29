use cgmath::Vector4;
use wgpu::util::DeviceExt;

use crate::{shader::{shader::{DruvisShader, ShaderPropertyTrait, ShaderBindState}, shader_property::{ShaderPropertyLayoutEntry, ShaderPropertyType, ShaderPropertyValue}}, binding::data_binding_state::DataBindingState, common::util_traits::AsBytes};

#[repr(C)]
pub struct DruvisColorShaderProperties {
    pub color: Vector4<f32>,
}

impl ShaderPropertyTrait for DruvisColorShaderProperties {
    fn get_bind_state(device: &wgpu::Device, label: &str) -> crate::shader::shader::ShaderBindState {
        let data = Self {
            color: Vector4::new(0.0, 0.0, 0.0, 0.0)
        };

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some((String::from(label) + "_buffer").as_str()),
                contents: data.druvis_as_bytes(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some((String::from(label) + "_bind_group_layout").as_str()),
            }
        );
        // println!("{:?}", bind_group_layout);

        ShaderBindState {
            value_bind_group_layout: bind_group_layout,
            // value_bind_group: bind_state.bind_group,
            value_buffer: buffer,
            // texture_bind_group_layout: None,
        }
    }

    fn get_shader_value_layout() -> Vec<crate::shader::shader_property::ShaderPropertyLayoutEntry> {
        vec![
            ShaderPropertyLayoutEntry {
                name: String::from("color"),
                ty: ShaderPropertyType::Vec4,
                default_value: ShaderPropertyValue::Vec4(Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 })
            }
        ]
    }

    fn get_shader_texture_layout() -> Vec<crate::shader::shader_property::ShaderTextureLayoutEntry> {
        vec![]
    }
}

pub struct DruvisColorShader;

impl DruvisColorShader {
    pub fn create_shader(
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) -> DruvisShader {
        let shader_module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("color_shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("color.wgsl").into())
            }
        );
        DruvisShader::new(
            device,
            "color_shader",
            builtin_bind_group_layouts,
            shader_module,
            color_format,
            depth_format,
            Some(wgpu::BlendState {
                alpha: wgpu::BlendComponent::REPLACE,
                color: wgpu::BlendComponent::REPLACE,
            }),
            wgpu::Face::Back,
            false,
            None,
            DruvisColorShaderProperties::get_shader_value_layout(),
            DruvisColorShaderProperties::get_shader_texture_layout(),
            DruvisColorShaderProperties::get_bind_state(device, "color_shader"),
            "druvis/color"
        )
    }
}
