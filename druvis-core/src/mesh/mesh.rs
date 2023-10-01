use cgmath::Point3;
use wgpu::util::DeviceExt;

use crate::{vertex::vertex::ModelVertex, common::util_traits::AsBytes, utils};

pub struct DruvisMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    // number of indices
    pub num_elements: u32,
    pub name: String,
    pub submeshes: Vec<(u64, u64)>,
}

impl DruvisMesh {
    pub fn get_submesh_count(&self) -> usize {
        if self.submeshes.len() == 0 || self.submeshes.len() == 1 {
            1
        } else {
            self.submeshes.len()
        }
    }

    pub fn get_submesh_index_count(&self, submesh_index: usize) -> u64 {
        let (start, end) = self.submeshes[submesh_index];
        end - start
    }

    pub fn get_index_buffer_slice(&self, index: usize) -> wgpu::BufferSlice {
        if index >= self.submeshes.len() {
            return self.index_buffer.slice(..);
        }

        let (start, end) = self.submeshes[index];
        // because u32 has 4bytes
        self.index_buffer.slice(start * 4..end * 4)
        // self.index_buffer.slice(..)
    }

    pub fn new(
        device: &wgpu::Device,
        label: &str,
        vertices: Vec<ModelVertex>,
        indices: Vec<u32>,
        submeshes: Vec<(u64, u64)>
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some((String::from(label) + "_vertex_buffer").as_str()),
                contents: utils::reinterpret_slice::<ModelVertex, u8>(&vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some((String::from(label) + "_index_buffer").as_str()),
                contents: utils::reinterpret_slice::<u32, u8>(&indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        Self {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            name: String::from(label),
            submeshes
        }
    }

    // pub fn from_vertices_and_indices(vertices: )
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
        for i in 0..indices.len() {
            indices[i] -= 1;
        }

        // println!("model vertex size: {}", std::mem::size_of::<ModelVertex>());
        // println!("{}", vertices.as_bytes().len());
        // println!("{:?}", utils::reinterpret_slice::<ModelVertex, f32>(&vertices));

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("cube_vertex_buffer"),
                contents: utils::reinterpret_slice::<ModelVertex, u8>(&vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("cube_index_buffer"),
                contents: indices.druvis_as_bytes(),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        DruvisMesh {
            vertex_buffer,
            index_buffer,
            num_elements: 36,
            name: String::from("cube"),
            submeshes: Vec::new(),
        }
    }
}
