use crate::{game_object::{components::MeshRendererData, TransformComponentData}, camera::camera::GetCameraUniform, instance::instance::DruvisInstance};

use super::render_pipeline::DruvisRenderPipeline;

pub struct SimpleRenderPipeline;

impl SimpleRenderPipeline {
    pub fn new() -> Self {
        Self
    }
}

impl DruvisRenderPipeline for SimpleRenderPipeline {
    fn render(&self, ins: &mut DruvisInstance) {
        // let device = &ins.device;
        let queue = &ins.queue;

        // update per frame uniforms
        ins.render_state.per_frame_data.camera_uniform = ins.camera.get_camera_uniform();
        ins.render_state.write_per_frame_buffer(queue);

        let components = ins.scene.get_components::<MeshRendererData>();

        let output = ins.surface.as_ref().unwrap().get_current_texture().unwrap();
        // let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        ins.render_state.set_color_target(&output.texture);
        
        for comp in components.iter() {
            let mesh_renderer = &comp.borrow().data;
            let transform = comp.borrow().get_component::<TransformComponentData>().unwrap();
            
            if mesh_renderer.mesh.is_none() | mesh_renderer.material.is_none() {
                continue;
            }

            let mesh = mesh_renderer.mesh.as_ref().unwrap();
            let material = mesh_renderer.material.as_ref().unwrap();

            ins.render_state.draw_mesh(
                &ins.device,
                &ins.queue,
                &mesh.borrow(),
                &material.borrow(),
                transform.borrow().data.get_model_matrix(),
                None
            );
        }

        output.present();
    }
}
