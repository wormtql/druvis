use std::ops::Range;

use crate::{material::material::DruvisMaterial, mesh::mesh::DruvisMesh};

pub trait DruvisDrawModel<'a> {
    fn draw_mesh(&mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial);

    fn draw_mesh_instanced(&mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial, instances: Range<u32>);
}

impl<'a, 'b> DruvisDrawModel<'a> for wgpu::RenderPass<'b> where 'a: 'b
{
    fn draw_mesh(&mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial) {
        self.draw_mesh_instanced(mesh, material, 0..1);
    }

    fn draw_mesh_instanced(&mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial, instances: Range<u32>) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        material.use_material(self);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
