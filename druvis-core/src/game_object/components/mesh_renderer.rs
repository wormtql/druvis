use std::{rc::Rc, cell::RefCell};

use crate::{material::material::DruvisMaterial, mesh::mesh::DruvisMesh};

pub struct MeshRendererData {
    pub material: Option<Rc<RefCell<DruvisMaterial>>>,
    pub mesh: Option<Rc<RefCell<DruvisMesh>>>,
}

impl Default for MeshRendererData {
    fn default() -> Self {
        Self {
            material: None,
            mesh: None,
        }
    }
}

// pub trait MeshRenderExt {
    // fn draw_mesh_renderer(&mut self, mesh_renderer: &MeshRendererData);
// }