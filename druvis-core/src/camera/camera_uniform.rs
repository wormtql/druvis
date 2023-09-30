use cgmath::{Matrix4, SquareMatrix};

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CameraUniform {
    pub druvis_world_space_camera_position: [f32; 4],
    pub druvis_view_matrix: [[f32; 4]; 4],
    pub druvis_projection_matrix: [[f32; 4]; 4],
    // same as unity
    pub druvis_projection_params: [f32; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        CameraUniform {
            druvis_world_space_camera_position: [0.0; 4],
            druvis_view_matrix: Matrix4::identity().into(),
            druvis_projection_matrix: Matrix4::identity().into(),
            druvis_projection_params: [1.0, 0.0, 0.0, 0.0],
        }
    }
}

pub trait UpdateCameraUniform {
    fn update_uniform(&self, uniform: &mut CameraUniform);
}
