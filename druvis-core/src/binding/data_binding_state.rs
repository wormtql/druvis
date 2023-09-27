use wgpu::util::DeviceExt;

use crate::common::util_traits::AsBytes;

pub struct DataBindingState<T> {
    pub data: T,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
}

impl<T: AsBytes> DataBindingState<T> {
    pub fn new(device: &wgpu::Device, data: T, label: &str) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some((String::from(label) + "_buffer").as_str()),
                contents: data.as_bytes(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some((String::from(label) + "_bind_group_layout").as_str()),
            }
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ],
                label: Some((String::from(label) + "_bind_group").as_str())
            }
        );

        Self {
            data,
            bind_group,
            bind_group_layout,
            buffer
        }
    }

    pub fn write_queue(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, self.data.as_bytes())
    }
}