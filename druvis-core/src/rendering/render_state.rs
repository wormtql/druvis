use cgmath::Matrix4;

use crate::{mesh::mesh::DruvisMesh, material::material::DruvisMaterial, binding::{data_binding_state::DataBindingState, bind_index::{BIND_GROUP_INDEX_PER_FRAME, BIND_GROUP_INDEX_PER_OBJECT}}, common::util_traits::AsBytes};

use super::uniform::{PerFrameUniform, PerObjectUniform};

pub struct RenderState {
    pub color_target: Option<wgpu::TextureView>,
    pub color_format: Option<wgpu::TextureFormat>,

    pub depth_target: Option<wgpu::TextureView>,
    pub depth_format: Option<wgpu::TextureFormat>,

    // pub surface: wgpu::Surface,
    // pub surface_config: wgpu::SurfaceConfiguration,
    // pub queue: wgpu::Queue,
    // pub device: wgpu::Device,

    pub per_frame_data: PerFrameUniform,
    pub per_frame_bind_state: DataBindingState,

    pub per_object_data: PerObjectUniform,
    pub per_object_bind_state: DataBindingState,
}

impl RenderState {
    pub async fn new(device: &wgpu::Device) -> Self {
        // let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        //     backends: wgpu::Backends::all(),
        //     dx12_shader_compiler: Default::default()
        // });

        // let adapter = instance.request_adapter(
        //     &wgpu::RequestAdapterOptions {
        //         power_preference: wgpu::PowerPreference::default(),
        //         compatible_surface,
        //         force_fallback_adapter: false
        //     }
        // ).await.unwrap();

        // let (device, queue) = adapter.request_device(
        //     &wgpu::DeviceDescriptor {
        //         features: wgpu::Features::empty(),
        //         limits: if cfg!(target_arch = "wasm32") {
        //             wgpu::Limits::downlevel_webgl2_defaults()
        //         } else {
        //             wgpu::Limits::default()
        //         },
        //         label: None,
        //     },
        //     None
        // ).await.unwrap();

        let per_frame_data = PerFrameUniform::default();
        let per_frame_bind_state = DataBindingState::new(&device, &per_frame_data, "per_frame_data");
        
        let per_object_data = PerObjectUniform::default();
        let per_object_bind_state = DataBindingState::new(&device, &per_object_data, "per_object_data");

        Self {
            // queue,
            // device,

            color_target: None,
            color_format: None,
            depth_target: None,
            depth_format: None,

            per_frame_data,
            per_frame_bind_state,

            per_object_data,
            per_object_bind_state,
        }
    }
}

impl RenderState {
    pub fn write_per_frame_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.per_frame_bind_state.buffer, 0, self.per_frame_data.druvis_as_bytes());
    }

    pub fn write_per_object_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.per_object_bind_state.buffer, 0, self.per_object_data.druvis_as_bytes());
    }

    pub fn set_color_target(&mut self, color_target: &wgpu::Texture) {
        let f = color_target.format();
        self.color_target = Some(color_target.create_view(&Default::default()));
        self.color_format = Some(f);
    }

    pub fn set_depth_target(&mut self, depth_target: &wgpu::Texture) {
        let f = depth_target.format();
        self.depth_target = Some(depth_target.create_view(&Default::default()));
        self.depth_format = Some(f);
    }

    pub fn clear_depth(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("draw_mesh")
            }
        );

        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("render_pass"),
                    color_attachments: &[],
                    depth_stencil_attachment: self.depth_format.map(|_| {
                        wgpu::RenderPassDepthStencilAttachment {
                            view: self.depth_target.as_ref().unwrap(),
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(0),
                                store: true
                            })
                        }
                    })
                }
            );
        }

        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn draw_mesh(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mesh: &DruvisMesh,
        material: &DruvisMaterial,
        transform_matrix: Matrix4<f32>,
        submesh_index: Option<usize>
    ) {
        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("draw_mesh")
            }
        );

        // println!("depth size {}", self.dep)

        // set per object transform
        self.per_object_data.transform.druvis_matrix_m = transform_matrix;
        queue.write_buffer(&self.per_object_bind_state.buffer, 0, self.per_object_data.druvis_as_bytes());

        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("render_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: self.color_target.as_ref().unwrap(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        }
                        // ops: wgpu::Operations {
                        //     load: wgpu::LoadOp::Clear(wgpu::Color {
                        //         r: 0.2,
                        //         g: 0.2,
                        //         b: 0.3,
                        //         a: 1.0
                        //     }),
                        //     store: true,
                        // },
                    })],
                    depth_stencil_attachment: self.depth_format.map(|_| {
                        wgpu::RenderPassDepthStencilAttachment {
                            view: self.depth_target.as_ref().unwrap(),
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            }),
                            stencil_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true
                            })
                        }
                    })
                }
            );

            // bind per frame bind group
            render_pass.set_bind_group(BIND_GROUP_INDEX_PER_FRAME, &self.per_frame_bind_state.bind_group, &[]);
            // bind per object bind group
            render_pass.set_bind_group(BIND_GROUP_INDEX_PER_OBJECT, &self.per_object_bind_state.bind_group, &[]);

            // set vertex buffer
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            // set index buffer
            render_pass.set_index_buffer(
                if submesh_index.is_some() {
                    mesh.get_index_buffer_slice(submesh_index.unwrap())
                } else {
                    mesh.index_buffer.slice(..)
                },
                wgpu::IndexFormat::Uint32
            );

            // use material
            material.use_material(device, &mut render_pass, self.color_format.unwrap(), self.depth_format);
            material.update_buffer(queue);

            // draw 
            let count = if submesh_index.is_some() {
                mesh.get_submesh_index_count(submesh_index.unwrap()) as u32
            } else {
                mesh.num_elements
            };
            render_pass.draw_indexed(0..count, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}