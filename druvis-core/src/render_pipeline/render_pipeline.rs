use crate::{scene::scene::DruvisScene, camera::{perspective_camera::PerspectiveCamera, camera_uniform::CameraUniform}, binding::data_binding_state::DataBindingState, common::transformation_uniform::TransformationUniform, rendering::render_state::RenderState, instance::instance::DruvisInstance};

pub trait DruvisRenderPipeline {
    // fn render(&self, render_data: &RenderData<'a>);

    fn render(&self, instance: &mut DruvisInstance);
}
