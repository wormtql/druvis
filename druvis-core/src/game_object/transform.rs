use std::{rc::{Weak, Rc}, cell::RefCell};

use cgmath::{Point3, Quaternion, One, Matrix4, Vector3};

use super::component::DruvisComponent;

pub struct TransformComponentData {
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,

    pub parent: Option<Weak<RefCell<DruvisComponent<TransformComponentData>>>>,
    pub children: Vec<Rc<RefCell<DruvisComponent<TransformComponentData>>>>,
}

impl TransformComponentData {
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let rot: Matrix4<f32> = self.rotation.into();
        Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, self.position.z))
        * rot
        * Matrix4::from_scale(self.scale)
    }
}

impl Default for TransformComponentData {
    fn default() -> Self {
        Self {
            position: Point3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Quaternion::one(),
            scale: 1.0,
            parent: None,
            children: Vec::new()
        }
    }
}
