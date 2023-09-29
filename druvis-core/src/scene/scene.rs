use std::{rc::Rc, cell::RefCell, any::Any, collections::HashMap};

use cgmath::Vector4;
use wgpu::{BindGroupLayout, TextureFormat};

use crate::{game_object::{game_object::{DruvisGameObject, DruvisGameObjectExt}, DruvisComponent, components::MeshRendererData}, mesh::mesh::DruvisMesh, shader::{shader::DruvisShader, shader_property::ShaderPropertyValue, shader_manager::ShaderManager}, material::material::DruvisMaterial};

pub struct DruvisScene {
    pub objects: Vec<Rc<RefCell<DruvisGameObject>>>,
}

impl DruvisScene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, go: Rc<RefCell<DruvisGameObject>>) {
        self.objects.push(go);
    }

    pub fn get_components<T: Any>(&self) -> Vec<Rc<RefCell<DruvisComponent<T>>>> {
        let mut result = Vec::new();
        for item in self.objects.iter() {
            let component = DruvisGameObject::get_component::<T>(item.clone());
            if component.is_some() {
                result.push(component.as_ref().unwrap().clone());
            }
        }

        result
    }
}

impl DruvisScene {
    pub fn simple_test_scene(
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&BindGroupLayout],
        color_format: TextureFormat,
        depth_format: Option<TextureFormat>,
        shader_manager: &ShaderManager,
    ) -> DruvisScene {
        let mut go = DruvisGameObject::new();

        let mut mesh_renderer = DruvisComponent::<MeshRendererData>::default();
        mesh_renderer.data.mesh = Some(Rc::new(RefCell::new(DruvisMesh::create_cube_mesh(device))));

        let shader = shader_manager.get_shader(device, builtin_bind_group_layouts, "druvis.color").unwrap();

        // let shader = DruvisColorShader::create_shader(
        //     device,
        //     builtin_bind_group_layouts,
        //     color_format,
        //     depth_format
        // );
        let mut material = DruvisMaterial::create_material(
            device,
            shader,
            HashMap::new(),
            "simple_material"
        ).unwrap();
        material.set_property("color", ShaderPropertyValue::Vec4(Vector4::new(0.5, 0.2, 0.1, 1.0)));

        mesh_renderer.data.material = Some(Rc::new(RefCell::new(material)));

        go.add_component(mesh_renderer);

        let mut scene = DruvisScene::new();
        scene.add_object(go);

        scene
    }
}