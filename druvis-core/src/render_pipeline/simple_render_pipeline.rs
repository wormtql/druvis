use winit::dpi::PhysicalSize;

use crate::{game_object::{components::MeshRendererData, TransformComponentData}, camera::camera::GetCameraUniform, instance::instance::DruvisInstance, texture::texture::DruvisTexture};

use super::render_pipeline::DruvisRenderPipeline;

pub struct SimpleRenderPipeline {
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
}

impl SimpleRenderPipeline {
    pub fn resize(&mut self, device: &wgpu::Device, new_size: PhysicalSize<u32>) {
        let (t, tv) = Self::create_depth_texture(device, wgpu::Extent3d {
            width: new_size.width,
            height: new_size.height,
            depth_or_array_layers: 1
        });

        self.depth_texture = t;
        self.depth_texture_view = tv;
    }

    fn create_depth_texture(device: &wgpu::Device, size: wgpu::Extent3d) -> (wgpu::Texture, wgpu::TextureView) {
        let depth_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("depth_texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[]
            }
        );
        let texture_view = depth_texture.create_view(&Default::default());

        (depth_texture, texture_view)
    }

    pub fn new(device: &wgpu::Device, size: wgpu::Extent3d) -> Self {
        let (t, tv) = Self::create_depth_texture(device, size);

        Self {
            depth_texture: t,
            depth_texture_view: tv
        }
    }
}

impl DruvisRenderPipeline for SimpleRenderPipeline {
    fn render(&self, ins: &mut DruvisInstance) {
        // let device = &ins.device;
        let queue = &ins.queue;

        // update per frame uniforms
        ins.render_state.per_frame_data.camera_uniform = ins.camera.get_camera_uniform();
        ins.render_state.write_per_frame_buffer(queue);
        // println!("{:?}", ins.render_state.per_frame_data.camera_uniform);

        let components = ins.scene.as_ref().unwrap().get_components::<MeshRendererData>();

        let output = ins.surface.as_ref().unwrap().get_current_texture().unwrap();
        // let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        ins.render_state.set_color_target(&output.texture);
        ins.render_state.set_depth_target(&self.depth_texture);

        ins.render_state.clear_depth(&ins.device, &ins.queue);
        
        for comp in components.iter() {
            // let mesh_renderer = &comp.borrow().data;
            // let transform = comp.borrow().get_component::<TransformComponentData>().unwrap();
            
            comp.borrow().draw_mesh_renderer(&ins.device, &ins.queue, &mut ins.render_state);

            // if mesh_renderer.mesh.is_none() | mesh_renderer.material.is_none() {
            //     continue;
            // }

            // let mesh = mesh_renderer.mesh.as_ref().unwrap();
            // let material = mesh_renderer.material.as_ref().unwrap();

            // ins.render_state.draw_mesh(
            //     &ins.device,
            //     &ins.queue,
            //     &mesh.borrow(),
            //     &material.borrow(),
            //     transform.borrow().data.get_model_matrix(),
            //     None
            // );
        }

        output.present();
    }
}
