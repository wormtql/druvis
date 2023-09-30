use std::path::Path;

use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent, KeyboardInput, MouseButton, ElementState, VirtualKeyCode, DeviceEvent}};

use crate::{camera::{perspective_camera::{PerspectiveCamera, SimplePerspectiveCameraController}, camera::GetCameraUniform, camera_uniform::CameraUniform}, render_pipeline::{simple_render_pipeline::SimpleRenderPipeline, render_pipeline::{DruvisRenderPipeline}}, scene::scene::DruvisScene, binding::data_binding_state::DataBindingState, common::transformation_uniform::TransformationUniform, shader::shader_manager::ShaderManager, rendering::{render_state::RenderState, uniform::{PerFrameUniform, PerObjectUniform}}};

pub struct DruvisInstance {
    // device and surface
    pub surface: Option<wgpu::Surface>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    // render
    pub render_state: RenderState,

    // window
    pub window: winit::window::Window,

    // world objects
    pub camera: PerspectiveCamera,
    pub camera_controller: SimplePerspectiveCameraController,
    pub scene: DruvisScene,

    // resource managers
    pub shader_manager: ShaderManager,
}

impl DruvisInstance {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            // WindowEvent::KeyboardInput {
            //     input:
            //         KeyboardInput {
            //             virtual_keycode: Some(key),
            //             state,
            //             ..
            //         },
            //     ..
            // } => self.camera_controller.process_keyboard(*key, *state),
            // WindowEvent::MouseWheel { delta, .. } => {
            //     self.camera_controller.process_scroll(delta);
            //     true
            // }
            // WindowEvent::MouseInput {
            //     button: MouseButton::Left,
            //     state,
            //     ..
            // } => {
            //     self.mouse_pressed = *state == ElementState::Pressed;
            //     true
            // }
            _ => false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            // self.size = new_size;
            // self.config.width = new_size.width;
            // self.config.height = new_size.height;
            // self.surface.configure(&self.device, &self.config);
            // self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            // self.projection.resize(new_size.width, new_size.height);
        }
    }

    pub fn render(&mut self, pipeline: &SimpleRenderPipeline) -> Result<(), wgpu::SurfaceError> {

        pipeline.render(self);

        Ok(())
    }

    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None
        ).await.unwrap();

        let render_state = RenderState::new(&device).await;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![]
        };
        surface.configure(&device, &config);

        let camera = PerspectiveCamera::default();
        let camera_controller = SimplePerspectiveCameraController::default();

        // let render_pipeline = SimpleRenderPipeline;

        let mut shader_manager = ShaderManager::new();
        shader_manager.add_search_path(Path::new("E:\\rust\\druvis\\druvis-core\\shaders").to_path_buf());

        let scene = DruvisScene::simple_test_scene(
            &device,
            &[
                &PerFrameUniform::get_bind_group_layout(&device),
                &PerObjectUniform::get_bind_group_layout(&device),
            ],
            surface_format,
            None,
            &shader_manager,
        );

        Self {
            surface: Some(surface),
            device,
            queue,
            surface_config: config,
            window,
            // event_loop,
            camera,
            camera_controller,
            // render_pipeline,
            scene,
            shader_manager,
            render_state,
        }
    }
}

pub async fn run() {
    let el = EventLoop::new();
    let window = WindowBuilder::new().build(&el).unwrap();

    let mut state = DruvisInstance::new(window).await;

    let rp = SimpleRenderPipeline::new();

    el.run(move |event, _, control_flow|  match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => if !state.input(event) {
            match event {
                #[cfg(not(target_arch="wasm32"))]
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { scale_factor: _, new_inner_size } => {
                    state.resize(**new_inner_size);
                }
                _ => {}
            }
        },
        Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            let now = instant::Instant::now();
            // let dt = now - last_render_time;
            // last_render_time = now;
            // state.update(dt);
            match state.render(&rp) {
                Ok(_) => {},
                // Err(wgpu::SurfaceError::Lost) => self.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            state.window.request_redraw();
        },
        // Event::DeviceEvent {
        //     event: DeviceEvent::MouseMotion{ delta, },
        //     .. // We're not using device_id currently
        // } => if state.mouse_pressed {
        //     state.camera_controller.process_mouse(delta.0, delta.1)
        // }
        _ => {}
    });
}