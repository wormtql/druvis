pub struct BindGroupLayoutBuilder {
    pub descriptors: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindGroupLayoutBuilder {
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub fn build(&self, device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &self.descriptors[..],
                label: Some(label),
            }
        )
    }

    pub fn add_buffer_entry(&mut self, device: &wgpu::Device, binding: u32) -> &mut Self {
        self.add_entry(
            wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        )
    }

    pub fn add_entry(&mut self, entry: wgpu::BindGroupLayoutEntry) -> &mut Self {
        self.descriptors.push(entry);
        self
    }

    pub fn add_sampler_entry(
        &mut self,
        sampler_binding: u32,
        sampler_type: wgpu::SamplerBindingType
    ) -> &mut Self {
        self.add_entry(
            wgpu::BindGroupLayoutEntry {
                binding: sampler_binding,
                visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Sampler(sampler_type),
                count: None
            }
        )
    }

    pub fn add_texture_entry(
        &mut self,
        texture_binding: u32,
        view_dimension: wgpu::TextureViewDimension,
    ) -> &mut Self {
        self.add_entry(
            wgpu::BindGroupLayoutEntry {
                binding: texture_binding,
                visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None
            }
        )
    }
}
