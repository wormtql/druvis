use crate::{game_object::{components::MeshRendererData, TransformComponentData}, rendering::rendering::DruvisDrawModel, common::{util_traits::AsBytes, transformation_uniform::TransformationUniform}, camera::camera::GetCameraUniform};

use super::render_pipeline::DruvisRenderPipeline;

pub struct SimpleRenderPipeline;

impl SimpleRenderPipeline {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> DruvisRenderPipeline<'a> for SimpleRenderPipeline {
    fn render(&self, render_data: &super::render_pipeline::RenderData<'a>) {
        // update camera uniform
        let queue = render_data.queue;
        let camera_uniform = render_data.camera.get_camera_uniform();
        queue.write_buffer(&render_data.camera_bind_state.buffer, 0, camera_uniform.as_bytes());

        let components = render_data.scene.get_components::<MeshRendererData>();

        let output = render_data.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        for comp in components.iter() {
            let mesh_renderer = &comp.borrow().data;
            let transform = comp.borrow().get_component::<TransformComponentData>().unwrap();
            
            if mesh_renderer.mesh.is_none() | mesh_renderer.material.is_none() {
                continue;
            }

            let mesh = mesh_renderer.mesh.as_ref().unwrap();
            let material = mesh_renderer.material.as_ref().unwrap();

            // write transformation uniform
            let transform_uniform = TransformationUniform {
                druvis_matrix_m: transform.borrow().data.get_model_matrix()
            };
            queue.write_buffer(&render_data.transform_bind_state.buffer, 0, transform_uniform.as_bytes());
            

            let mut encoder = render_data.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("render_encoder")
                }
            );

            let mesh_borrow = mesh.borrow();
            let material_borrow = material.borrow();
    
            {
                let mut render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("render_pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                        // depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        //     view: &self.depth_texture.view,
                        //     depth_ops: Some(wgpu::Operations {
                        //         load: wgpu::LoadOp::Clear(1.0),
                        //         store: true
                        //     }),
                        //     stencil_ops: None,
                        // })
                    }
                );
    
                render_pass.draw_mesh(&*mesh_borrow, &*material_borrow);
            }
            

            queue.submit(std::iter::once(encoder.finish()));
        }

        output.present();
    }
}
