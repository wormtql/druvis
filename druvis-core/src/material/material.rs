use crate::{shader::shader::DruvisShader, common::util_traits::AsBytes, binding::bind_index::BIND_GROUP_INDEX_SHADER_TEXTURE};

pub struct MaterialBindState {
    pub texture_bind_group: wgpu::BindGroup,
}

pub struct DruvisMaterial<'a, T> {
    pub shader: DruvisShader<'a>,
    pub properties: T,
    pub bind_state: MaterialBindState,
}

impl<'a, T: AsBytes> DruvisMaterial<'a, T> {
    pub fn use_material<'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b {
        render_pass.set_bind_group(BIND_GROUP_INDEX_SHADER_TEXTURE, &self.bind_state.texture_bind_group, &[]);
    }
 
    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.shader.shader_bind_state.value_buffer, 0, self.properties.as_bytes());
    }
}
