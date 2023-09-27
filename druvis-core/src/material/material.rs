use std::rc::Rc;

use crate::{shader::{shader::DruvisShader, shader_manager::ShaderManager}, common::util_traits::AsBytes, binding::{bind_index::BIND_GROUP_INDEX_SHADER_TEXTURE, bind_group_builder::BindGroupBuilder}, texture::texture::DruvisTextureAndSampler};

pub struct MaterialBindState {
    pub texture_bind_group: wgpu::BindGroup,
}

pub struct DruvisMaterial<'a, T> {
    pub name: String,
    pub shader: Rc<DruvisShader<'a>>,
    pub properties: T,
    pub textures: Vec<Rc<DruvisTextureAndSampler>>,
    pub bind_state: MaterialBindState,
}

impl<'a, T: AsBytes> DruvisMaterial<'a, T> {
    pub fn use_material<'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b {
        render_pass.set_bind_group(BIND_GROUP_INDEX_SHADER_TEXTURE, &self.bind_state.texture_bind_group, &[]);
    }
 
    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.shader.shader_bind_state.value_buffer, 0, self.properties.as_bytes());
    }

    pub fn create_material(
        device: &wgpu::Device,
        shader_manager: &ShaderManager<'a>,
        shader_name: &str,
        properties: T,
        textures: &[Rc<DruvisTextureAndSampler>],
        name: &str
    ) -> Option<Self> {
        let shader = shader_manager.get_shader(shader_name);
        if shader.is_none() {
            return None;
        }

        let mut bind_group_builder = BindGroupBuilder::new();
        let mut binding_index = 0_u32;
        for tex in textures.iter() {
            bind_group_builder.add_druvis_texture_and_sampler(binding_index, tex);
            binding_index += 2;
        }

        let shader = shader.unwrap();
        let mat = DruvisMaterial {
            bind_state: MaterialBindState {
                texture_bind_group: bind_group_builder.build(
                    device,
                    shader.shader_bind_state.texture_bind_group_layout.as_ref().unwrap(),
                    name
                )
            },
            name: String::from(name),
            shader,
            properties,
            textures: textures.iter().cloned().collect(),
        };

        Some(mat)
    }
}
