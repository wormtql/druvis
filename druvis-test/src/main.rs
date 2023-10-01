use std::{rc::Rc, cell::RefCell, collections::HashMap, path::Path};

use druvis_core::{instance::instance::DruvisInstance, render_pipeline::simple_render_pipeline::SimpleRenderPipeline, camera::camera::CameraController, scene::scene::DruvisScene, shader::shader_manager::ShaderManager, material::{material_manager::MaterialManager, material::DruvisMaterial}, game_object::{DruvisGameObject, DruvisComponent, components::MeshRendererData, game_object::DruvisGameObjectExt}, mesh::mesh::DruvisMesh};
use druvis_mmd_parser::PmxParser;
use winit::{event_loop::{EventLoop, ControlFlow}, window::*, event::*};

pub async fn run() {
    let el = EventLoop::new();
    let window = WindowBuilder::new().build(&el).unwrap();

    let mut state = DruvisInstance::new(window).await;

    let scene = create_scene(
        &state.device,
        &state.queue,
        &state.get_builtin_bind_group_layout_ref(),
        &state.shader_manager,
        &state.material_manager
    );
    state.scene = Some(scene);

    let mut rp = SimpleRenderPipeline::new(&state.device, wgpu::Extent3d {
        width: state.size.width,
        height: state.size.height,
        depth_or_array_layers: 1
    });

    let mut last_render_time = instant::Instant::now();

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

fn create_scene(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
    shader_manager: &ShaderManager,
    material_manager: &MaterialManager,
) -> DruvisScene {
    let model = include_bytes!("../../models/yoimiya/宵宫.pmx");
    let model_path = Path::new("E:\\rust\\druvis\\models\\yoimiya");
    let parser = PmxParser::new();

    let parse_result = parser.parse(model, model_path.to_path_buf()).unwrap();
    // let mesh = parse_result.to_druvis_mesh(device);

    // let go = DruvisGameObject::new();

    // let mut mesh_renderer = DruvisComponent::<MeshRendererData>::default();
    // mesh_renderer.data.mesh = Some(Rc::new(RefCell::new(mesh)));

    // let shader = shader_manager.get_shader(device, builtin_bind_group_layouts, "druvis.albedo").unwrap();
    // let material = DruvisMaterial::create_material(device, shader, HashMap::new(), "albedo").unwrap();

    // mesh_renderer.data.material = Some(Rc::new(RefCell::new(material)));

    // go.add_component(mesh_renderer);

    let go = parse_result.create_game_object(device, queue, shader_manager, builtin_bind_group_layouts);

    let mut scene = DruvisScene::new();
    scene.add_object(go);

    scene
}

fn main() {
    pollster::block_on(run());
}
