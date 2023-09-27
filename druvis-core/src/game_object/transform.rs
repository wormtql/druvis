use std::{rc::{Weak, Rc}, cell::RefCell};

use cgmath::{Point3, Quaternion, One};

use super::component::DruvisComponent;

pub struct TransformComponentData {
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,

    pub parent: Option<Weak<RefCell<DruvisComponent<TransformComponentData>>>>,
    pub children: Vec<Rc<RefCell<DruvisComponent<TransformComponentData>>>>,
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
