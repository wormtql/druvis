use crate::texture::texture::DruvisTextureAndSampler;

pub struct BindGroupBuilder<'a> {
    pub entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new() -> Self {
        BindGroupBuilder {
            entries: Vec::new(),
        }
    }

    pub fn add_buffer<'b>(&mut self, binding_index: u32, buffer: &'b wgpu::Buffer) -> &mut Self where 'b: 'a {
        self.entries.push(wgpu::BindGroupEntry {
            binding: binding_index,
            resource: buffer.as_entire_binding(),
        });
        self
    }

    pub fn add_texture(&mut self, binding_index: u32, texture_view: &'a wgpu::TextureView) -> &mut Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: binding_index,
            resource: wgpu::BindingResource::TextureView(texture_view)
        });
        self
    }

    pub fn add_sampler(&mut self, binding_index: u32, sampler: &'a wgpu::Sampler) -> &mut Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: binding_index,
            resource: wgpu::BindingResource::Sampler(sampler)
        });
        self
    }

    pub fn add_druvis_texture_and_sampler(&mut self, texture_binding_index: u32, texture_and_sampler: &'a DruvisTextureAndSampler) -> &mut Self {
        self.add_texture(texture_binding_index, &texture_and_sampler.texture.view);
        self.add_sampler(texture_binding_index + 1, &texture_and_sampler.sampler.sampler);
        self
    }

    pub fn build(self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str) -> wgpu::BindGroup {
        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                entries: &self.entries[..],
                layout,
                label: Some(label)
            }
        )
    }
}
