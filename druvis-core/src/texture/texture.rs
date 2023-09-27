pub struct DruvisSampler {
    pub sampler: wgpu::Sampler,
    pub sampler_type: wgpu::SamplerBindingType,
}

pub struct DruvisTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,

    pub dimension: wgpu::TextureDimension,
}

pub struct DruvisTextureAndSampler {
    pub sampler: DruvisSampler,
    pub texture: DruvisTexture,
}

