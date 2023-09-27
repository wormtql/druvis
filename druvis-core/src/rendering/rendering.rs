use std::ops::Range;

use crate::{material::material::DruvisMaterial, mesh::mesh::DruvisMesh};

pub trait DruvisDrawModel<'a, 'b> {
    fn draw_mesh<T>(&'a mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial<'b, T>);

    fn draw_mesh_instanced<T>(&'a mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial<'b, T>, instances: Range<u32>);
}

impl<'a, 'b, 'c> DruvisDrawModel<'a, 'b> for wgpu::RenderPass<'c>
where
    'a: 'c,
{
    fn draw_mesh<T>(&'a mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial<'b, T>) {
        self.draw_mesh_instanced(mesh, material, 0..1);
    }

    fn draw_mesh_instanced<T>(&'a mut self, mesh: &'a DruvisMesh, material: &'a DruvisMaterial<'b, T>, instances: Range<u32>) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        material.use_material(self);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
