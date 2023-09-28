use cgmath::Point3;
use wgpu::util::DeviceExt;

use crate::{vertex::vertex::ModelVertex, common::util_traits::AsBytes};

pub struct DruvisMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub name: String,
}

impl DruvisMesh {
    pub fn create_cube_mesh(device: &wgpu::Device) -> Self {
        let mut vertices = Vec::new();
        vertices.push(ModelVertex {
            position: [-0.5, -0.5, -0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [-0.5, 0.5, -0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [0.5, 0.5, -0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [0.5, -0.5, -0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [-0.5, -0.5, 0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [-0.5, 0.5, 0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [0.5, 0.5, 0.5],
            ..Default::default()
        });
        vertices.push(ModelVertex {
            position: [0.5, -0.5, 0.5],
            ..Default::default()
        });

        let mut indices: [u32; 36] = [
            1, 2, 3, 4, 1, 3,
            7, 8, 4, 3, 7, 4,
            6, 7, 3, 2, 6, 3,
            1, 6, 2, 1, 5, 6,
            4, 8, 5, 4, 5, 1,
            5, 8, 7, 5, 7, 6,
        ];

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("cube_vertex_buffer"),
                contents: vertices.as_bytes(),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("cube_index_buffer"),
                contents: indices.as_bytes(),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        DruvisMesh {
            vertex_buffer,
            index_buffer,
            num_elements: 36,
            name: String::from("cube")
        }
    }
}
