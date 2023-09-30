use cgmath::{Matrix4, Zero};

#[repr(C)]
#[derive(Debug)]
pub struct TransformationUniform {
    pub druvis_matrix_m: Matrix4<f32>,
}

impl Default for TransformationUniform {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformationUniform {
    pub fn new() -> Self {
        Self {
            druvis_matrix_m: Matrix4::zero()
        }
    }
}