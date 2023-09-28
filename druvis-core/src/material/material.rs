use std::{rc::Rc, collections::HashMap};

use crate::{shader::{shader::DruvisShader, shader_property::ShaderPropertyValue}, binding::{bind_index::BIND_GROUP_INDEX_SHADER_TEXTURE, bind_group_builder::BindGroupBuilder}, texture::texture::DruvisTextureAndSampler, utils};

pub struct MaterialBindState {
    pub texture_bind_group: wgpu::BindGroup,
}

pub struct DruvisMaterial {
    pub name: String,
    pub shader: Rc<DruvisShader>,
    pub properties: HashMap<String, ShaderPropertyValue>,
    pub textures: Vec<Rc<DruvisTextureAndSampler>>,
    pub bind_state: MaterialBindState,
}

impl DruvisMaterial {
    pub fn set_property(&mut self, key: &str, value: ShaderPropertyValue) {
        self.properties.insert(String::from(key), value);
    }

    pub fn use_material<'a, 'b>(&'a self, render_pass: &mut wgpu::RenderPass<'b>) where 'a: 'b {
        render_pass.set_bind_group(BIND_GROUP_INDEX_SHADER_TEXTURE, &self.bind_state.texture_bind_group, &[]);
    }
 
    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        let mut buffer_vec = utils::create_buffer(self.shader.shader_value_size);
        let buffer = buffer_vec.as_mut_slice();

        let mut offset = 0;
        for item in self.shader.shader_value_layout.iter() {
            let name = item.name.as_str();
            let value = self.properties.get(name);
            if value.is_some() {
                utils::write_buffer(buffer, offset, value.unwrap().get_bytes());
            } else {
                let default_value = item.default_value.get_bytes();
                utils::write_buffer(buffer, offset, default_value);
            }

            offset += item.ty.get_size();
        }

        queue.write_buffer(&self.shader.shader_bind_state.value_buffer, 0, buffer);
    }

    pub fn create_material(
        device: &wgpu::Device,
        shader: Rc<DruvisShader>,
        textures: &[Rc<DruvisTextureAndSampler>],
        name: &str
    ) -> Option<Self> {
        let mut bind_group_builder = BindGroupBuilder::new();
        let mut binding_index = 0_u32;
        for tex in textures.iter() {
            bind_group_builder.add_druvis_texture_and_sampler(binding_index, tex);
            binding_index += 2;
        }

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
            properties: HashMap::new(),
            textures: textures.iter().cloned().collect(),
        };

        Some(mat)
    }
}
