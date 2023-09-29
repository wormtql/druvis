use std::ops::Range;

use crate::{material::material::DruvisMaterial, mesh::mesh::DruvisMesh};

pub trait DruvisDrawModel<'a> {
    fn draw_mesh(
        &mut self,
        device: &wgpu::Device,
        mesh: &'a DruvisMesh,
        material: &'a DruvisMaterial,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) {
        self.draw_mesh_instanced(
            device,
            mesh,
            material,
            0..1,
            color_format,
            depth_format
        );
    }

    fn draw_mesh_instanced(
        &mut self,
        device: &wgpu::Device,
        mesh: &'a DruvisMesh,
        material: &'a DruvisMaterial,
        instances: Range<u32>,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    );
}

impl<'a, 'b> DruvisDrawModel<'a> for wgpu::RenderPass<'b> where 'a: 'b
{
    fn draw_mesh_instanced(
        &mut self,
        device: &wgpu::Device,
        mesh: &'a DruvisMesh,
        material: &'a DruvisMaterial,
        instances: Range<u32>,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        material.use_material(device, self, color_format, depth_format);
        // self.draw_indexed(0..3, 0, instances);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
