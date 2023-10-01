use std::path::{PathBuf, Path};

use wgpu::util::DeviceExt;

pub struct DruvisSampler {
    pub sampler: wgpu::Sampler,
    pub sampler_type: wgpu::SamplerBindingType,
}

impl DruvisSampler {
    pub fn new(device: &wgpu::Device, desc: &wgpu::SamplerDescriptor, ty: wgpu::SamplerBindingType) -> Self {
        let sampler = device.create_sampler(desc);
        Self {
            sampler,
            sampler_type: ty,
        }
    }
}

pub struct DruvisTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,

    pub dimension: wgpu::TextureDimension,
}

impl DruvisTexture {
    pub fn from_path(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &Path,
        format: wgpu::TextureFormat,
    ) -> Self {
        let img = image::open(path).unwrap();
        let rgba = img.to_rgba8();

        let width = img.width();
        let height = img.height();
        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let filename = path.file_name().unwrap().to_str();

        Self::new_2d(
            device,
            queue,
            &rgba,
            extent,
            format,
            filename.unwrap(),
        )
    }

    pub fn new_2d(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_data: &[u8],
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
        label: &str
    ) -> Self {
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[]
            }
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO
            },
            image_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            size
        );

        let view = texture.create_view(&Default::default());

        Self {
            texture,
            view,
            dimension: wgpu::TextureDimension::D2,
        }
    }
}

pub struct DruvisTextureAndSampler {
    pub sampler: DruvisSampler,
    pub texture: DruvisTexture,
}

impl DruvisTextureAndSampler {
    pub fn new_2d(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_data: &[u8],
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
        sampler_desc: &wgpu::SamplerDescriptor,
        sampler_binding_type: wgpu::SamplerBindingType,
        label: &str
    ) -> Self {
        let druvis_texture = DruvisTexture::new_2d(device, queue, image_data, size, format, label);
        let druvis_sampler = DruvisSampler::new(device, sampler_desc, sampler_binding_type);

        Self {
            sampler: druvis_sampler,
            texture: druvis_texture
        }
    }

    pub fn from_path(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &Path,
        format: wgpu::TextureFormat,
        sampler_desc: &wgpu::SamplerDescriptor,
        sampler_binding_type: wgpu::SamplerBindingType,
    ) -> Self {
        let druvis_texture = DruvisTexture::from_path(device, queue, path, format);
        let druvis_sampler = DruvisSampler::new(device, sampler_desc, sampler_binding_type);

        Self {
            sampler: druvis_sampler,
            texture: druvis_texture
        }
    }
}
