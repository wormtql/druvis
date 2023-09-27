use std::{collections::HashMap, rc::Rc};

use super::{shader::DruvisShader, builtin_shaders::DruvisColorShader};

pub struct ShaderManager<'a> {
    pub loaded_shaders: HashMap<String, Rc<DruvisShader<'a>>>
}

impl<'a> ShaderManager<'a> {
    pub fn load_builtin_shaders(
        &mut self,
        device: &'a wgpu::Device,
        builtin_bind_group_layouts: &'a [wgpu::BindGroupLayout],
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) {
        let color_shader = DruvisColorShader::create_shader(
            device,
            builtin_bind_group_layouts,
            color_format,
            depth_format
        );
        self.loaded_shaders.insert(color_shader.name.clone(), Rc::new(color_shader));
    }

    pub fn new() -> Self {
        Self {
            loaded_shaders: HashMap::new()
        }
    }

    pub fn get_shader(&self, name: &str) -> Option<Rc<DruvisShader<'a>>> {
        self.loaded_shaders.get(name).cloned()
    }
}
