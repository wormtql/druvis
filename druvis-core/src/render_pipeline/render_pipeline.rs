use crate::{scene::scene::DruvisScene, camera::{perspective_camera::PerspectiveCamera, camera_uniform::CameraUniform}, binding::data_binding_state::DataBindingState, common::transformation_uniform::TransformationUniform};

pub struct RenderData<'a> {
    pub device: &'a wgpu::Device,
    pub scene: &'a DruvisScene,
    pub queue: &'a wgpu::Queue,
    pub surface: &'a wgpu::Surface,
    pub surface_config: &'a wgpu::SurfaceConfiguration,
    pub camera: &'a PerspectiveCamera,
    pub transform_bind_state: &'a DataBindingState,
    pub camera_bind_state: &'a DataBindingState,
    pub camera_uniform: &'a CameraUniform,
}

pub trait DruvisRenderPipeline<'a> {
    fn render(&self, render_data: &RenderData<'a>);
}
