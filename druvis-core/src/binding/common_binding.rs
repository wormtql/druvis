use cgmath::{Matrix4, Vector4};

pub enum CommonBindingItem {
    Matrix4(Matrix4<f32>),
    Vector4(Vector4<f32>),
}

