use std::{rc::Rc, cell::RefCell};

use crate::{material::material::DruvisMaterial, mesh::mesh::DruvisMesh, rendering::render_state::RenderState, game_object::{DruvisComponent, TransformComponentData}};

pub struct MeshRendererData {
    pub mesh: Option<Rc<RefCell<DruvisMesh>>>,
    pub materials: Vec<Rc<RefCell<DruvisMaterial>>>,
}

impl Default for MeshRendererData {
    fn default() -> Self {
        Self {
            materials: Vec::new(),
            mesh: None,
        }
    }
}

impl DruvisComponent<MeshRendererData> {
    pub fn draw_mesh_renderer(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_state: &mut RenderState
    ) {
        if self.data.mesh.is_none() || self.data.materials.len() == 0 {
            return;
        }

        let transform = self.get_component::<TransformComponentData>();
        if transform.is_none() {
            panic!("Cannot draw mesh renderer without transform component");
        }

        let mesh = self.data.mesh.as_ref().unwrap().clone();
        if mesh.borrow().get_submesh_count() == 1 {
            render_state.draw_mesh(
                device,
                queue,
                &mesh.borrow(),
                &self.data.materials[0].borrow(),
                transform.as_ref().unwrap().borrow().data.get_model_matrix(),
                None
            )
        } else {
            let submesh_count = mesh.borrow().get_submesh_count();
            let transform_matrix = transform.as_ref().unwrap().borrow().data.get_model_matrix();

            // render_state.draw_mesh(
            //     device,
            //     queue,
            //     &mesh.borrow(),
            //     &self.data.materials[0].borrow(),
            //     transform_matrix,
            //     Some(0)
            // )

            for i in 0..submesh_count {
                render_state.draw_mesh(
                    device,
                    queue,
                    &mesh.borrow(),
                    &self.data.materials[i].borrow(),
                    transform_matrix,
                    Some(i)
                )
            }
        }
    }
}