use std::path::Path;

use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent, KeyboardInput, MouseButton, ElementState, VirtualKeyCode, DeviceEvent}, dpi::PhysicalSize};

use crate::{camera::{perspective_camera::{PerspectiveCamera, SimplePerspectiveCameraController}, camera::{GetCameraUniform, CameraController}, camera_uniform::CameraUniform}, render_pipeline::{simple_render_pipeline::SimpleRenderPipeline, render_pipeline::{DruvisRenderPipeline}}, scene::scene::DruvisScene, binding::data_binding_state::DataBindingState, common::transformation_uniform::TransformationUniform, shader::shader_manager::ShaderManager, rendering::{render_state::RenderState, uniform::{PerFrameUniform, PerObjectUniform}}, material::material_manager::MaterialManager};

pub struct DruvisInstance {
    // device and surface
    pub surface: Option<wgpu::Surface>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: PhysicalSize<u32>,
    pub builtin_bind_group_layouts: Vec<wgpu::BindGroupLayout>,

    // render
    pub render_state: RenderState,

    // window
    pub window: winit::window::Window,

    // world objects
    pub camera: PerspectiveCamera,
    pub scene: Option<DruvisScene>,

    // camera control
    pub camera_controller: SimplePerspectiveCameraController,
    pub mouse_pressed: bool,

    // resource managers
    pub shader_manager: ShaderManager,
    pub material_manager: MaterialManager,
}

impl DruvisInstance {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 && self.surface.is_some() {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.as_ref().unwrap().configure(&self.device, &self.surface_config);

            // reset camera aspect
            self.camera.aspect = new_size.width as f32 / new_size.height as f32;

            // self.config.width = new_size.width;
            // self.config.height = new_size.height;
            // self.surface.configure(&self.device, &self.config);
            // self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            // self.projection.resize(new_size.width, new_size.height);
        }
    }

    pub fn update(&mut self, delta_time: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera, delta_time)
    }

    pub fn render(&mut self, pipeline: &SimpleRenderPipeline) -> Result<(), wgpu::SurfaceError> {

        pipeline.render(self);

        Ok(())
    }

    pub fn get_builtin_bind_group_layout_ref(&self) -> Vec<&wgpu::BindGroupLayout> {
        self.builtin_bind_group_layouts.iter().collect()
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

        let camera = PerspectiveCamera::new(
            (0.0, 0.0, 1.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
            size.width as f32 / size.height as f32,
            cgmath::Deg(45.0),
            0.1,
            100.0
        );
        let camera_controller = SimplePerspectiveCameraController::new(4.0, 1.0);

        // let render_pipeline = SimpleRenderPipeline;

        let mut shader_manager = ShaderManager::new();
        // todo use more robust path
        shader_manager.add_search_path(Path::new("E:\\rust\\druvis\\druvis-core\\shaders").to_path_buf());

        let mut material_manager = MaterialManager::new();
        material_manager.add_search_path(Path::new("E:\\rust\\druvis\\druvis-core\\materials").to_path_buf());

        // let scene = DruvisScene::simple_test_scene(
        //     &device,
        //     &[
        //         &PerFrameUniform::get_bind_group_layout(&device),
        //         &PerObjectUniform::get_bind_group_layout(&device),
        //     ],
        //     &shader_manager,
        //     &material_manager,
        // );

        let builtin_bind_group_layouts = vec![
            PerFrameUniform::get_bind_group_layout(&device),
            PerObjectUniform::get_bind_group_layout(&device),
        ];

        Self {
            surface: Some(surface),
            device,
            queue,
            surface_config: config,
            window,
            size,
            // event_loop,
            camera,
            camera_controller,
            // render_pipeline,
            scene: None,
            shader_manager,
            material_manager,
            render_state,
            mouse_pressed: false,
            builtin_bind_group_layouts
        }
    }
}

pub async fn run() {
    let el = EventLoop::new();
    let window = WindowBuilder::new().build(&el).unwrap();

    let mut state = DruvisInstance::new(window).await;

    let mut rp = SimpleRenderPipeline::new(&state.device, wgpu::Extent3d {
        width: state.size.width,
        height: state.size.height,
        depth_or_array_layers: 1,
    });

    let mut last_render_time = instant::Instant::now();

    let scene = DruvisScene::simple_test_scene(
        &state.device,
        &state.get_builtin_bind_group_layout_ref(),
        &state.shader_manager,
        &state.material_manager,
    );
    state.scene = Some(scene);

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
                    println!("resized event");
                    state.resize(*physical_size);
                    rp.resize(&state.device, *physical_size);
                }
                WindowEvent::ScaleFactorChanged { scale_factor: _, new_inner_size } => {
                    state.resize(**new_inner_size);
                    rp.resize(&state.device, **new_inner_size);
                }
                _ => {}
            }
        },
        Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            let now = instant::Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;
            state.update(dt);
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
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion{ delta, },
            .. // We're not using device_id currently
        } => if state.mouse_pressed {
            state.camera_controller.process_mouse(delta.0, delta.1)
        }
        _ => {}
    });
}