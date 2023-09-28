use cgmath::{Matrix4, Zero};

#[repr(C)]
pub struct TransformationUniform {
    pub druvis_matrix_m: Matrix4<f32>,
}

impl TransformationUniform {
    pub fn new() -> Self {
        Self {
            druvis_matrix_m: Matrix4::zero()
        }
    }
}