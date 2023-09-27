use cgmath::Vector4;

use crate::{shader::shader::{DruvisShader, ShaderPropertyTrait, ShaderBindState}, binding::data_binding_state::DataBindingState};

#[repr(C)]
pub struct DruvisColorShaderProperties {
    pub color: Vector4<f32>,
}

impl ShaderPropertyTrait for DruvisColorShaderProperties {
    fn get_bind_state(device: &wgpu::Device, label: &str) -> crate::shader::shader::ShaderBindState {
        let data = Self {
            color: Vector4::new(0.0, 0.0, 0.0, 0.0)
        };
        let bind_state = DataBindingState::new(device, data, label);
        ShaderBindState {
            value_bind_group_layout: bind_state.bind_group_layout,
            value_bind_group: bind_state.bind_group,
            value_buffer: bind_state.buffer,
            texture_bind_group_layout: None,
        }
    }
}

pub struct DruvisColorShader;

impl DruvisColorShader {
    pub fn create_shader<'a>(
        device: &'a wgpu::Device,
        builtin_bind_group_layouts: &'a [wgpu::BindGroupLayout],
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) -> DruvisShader<'a> {
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
            DruvisColorShaderProperties::get_bind_state(device, "color_shader"),
            "druvis/color"
        )
    }
}
