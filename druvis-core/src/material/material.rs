use std::{rc::Rc, collections::HashMap, cell::RefCell};

use crate::{shader::{shader::DruvisShader, shader_property::ShaderPropertyValue, shader_manager::ShaderManager}, binding::{bind_index::{BIND_GROUP_SHADER_PROPERTIES}, bind_group_builder::BindGroupBuilder}, texture::texture::DruvisTextureAndSampler, utils};

use super::material_descriptor::MaterialDescriptor;

pub struct MaterialBindState {
    // pub texture_bind_group: wgpu::BindGroup,
    pub shader_properties_bind_group: wgpu::BindGroup,
}

pub struct DruvisMaterial {
    pub name: String,
    pub shader: Rc<DruvisShader>,
    pub properties: HashMap<String, ShaderPropertyValue>,
    pub texture_properties: HashMap<String, Rc<DruvisTextureAndSampler>>,
    // pub textures: Vec<Rc<DruvisTextureAndSampler>>,
    pub bind_state: MaterialBindState,
}

impl DruvisMaterial {
    pub fn from_descriptor(
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_manager: &ShaderManager,
        desc: &MaterialDescriptor
    ) -> Option<Self> {
        let mut textures = HashMap::new();

        // set textures
        // todo
        
        let shader = shader_manager.get_shader(device, builtin_bind_group_layouts, &desc.shader_name)?;
        
        let mut mat = DruvisMaterial::create_material(device, shader, textures, &desc.name)?;

        // set properties
        for prop in desc.properties.iter() {
            mat.set_property(prop.0, prop.1.clone());
        }

        Some(mat)
    }
    
    pub fn set_property(&mut self, key: &str, value: ShaderPropertyValue) {
        let mut flag = false;
        for item in self.shader.shader_value_layout.iter() {
            if item.name == key {
                flag = true;
                break;
            }
        }

        if flag {
            self.properties.insert(String::from(key), value);
        } else {
            panic!("Shader property {} does not exist", key);
        }
    }

    pub fn set_texture_property(&mut self, key: &str, value: Rc<DruvisTextureAndSampler>) {
        self.texture_properties.insert(String::from(key), value);
    }

    pub fn use_material<'a, 'b>(
        &'a self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass<'b>,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) where 'a: 'b {
        self.shader.use_shader(device, render_pass, color_format, depth_format);
        render_pass.set_bind_group(BIND_GROUP_SHADER_PROPERTIES, &self.bind_state.shader_properties_bind_group, &[]);
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
        textures: HashMap<String, Rc<DruvisTextureAndSampler>>,
        name: &str
    ) -> Option<Self> {
        let mut bind_group_builder = BindGroupBuilder::new();
        let mut binding_index = 1_u32;

        // add value buffer
        bind_group_builder.add_buffer(0, &shader.shader_bind_state.value_buffer);

        for tex_layout_entry in shader.shader_texture_layout.iter() {
            // todo use a default black texture
            let tex = textures.get(&tex_layout_entry.name).unwrap();
            bind_group_builder.add_druvis_texture_and_sampler(binding_index, tex);
            binding_index += 2;
        }

        let mat = DruvisMaterial {
            bind_state: MaterialBindState {
                shader_properties_bind_group: bind_group_builder.build(
                    device,
                    &shader.shader_bind_state.value_bind_group_layout,
                    name
                )
            },
            name: String::from(name),
            shader,
            properties: HashMap::new(),
            // textures: textures.iter().cloned().collect(),
            texture_properties: HashMap::new(),
        };

        Some(mat)
    }
}
