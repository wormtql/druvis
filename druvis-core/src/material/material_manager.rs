use std::{collections::{HashMap, HashSet}, path::PathBuf, rc::Rc, cell::RefCell};

use crate::shader::shader_manager::ShaderManager;

use super::{material::DruvisMaterial, material_descriptor::MaterialDescriptor};

pub struct MaterialManager {
    pub loaded_material: HashMap<String, Rc<RefCell<DruvisMaterial>>>,
    pub failed_material: HashSet<String>,

    pub search_list: Vec<PathBuf>,
}

impl MaterialManager {
    pub fn new() -> Self {
        MaterialManager {
            loaded_material: HashMap::new(),
            failed_material: HashSet::new(),
            search_list: Vec::new()
        }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_list.push(path);
    }

    pub fn get_material(
        &self,
        name: &str,
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_manager: &ShaderManager,
    ) -> Option<Rc<RefCell<DruvisMaterial>>> {
        if self.loaded_material.contains_key(name) {
            return self.loaded_material.get(name).cloned()
        }
        if self.failed_material.contains(name) {
            return None;
        }

        let mat = self.load_material(name, device, builtin_bind_group_layouts, shader_manager);
        if mat.is_some() {
            unsafe {
                let ptr = &self.loaded_material as *const _;
                let ptr = ptr as *mut HashMap<String, Rc<RefCell<DruvisMaterial>>>;

                let m = Rc::new(RefCell::new(mat.unwrap()));
                (*ptr).insert(String::from(name), m.clone());

                return Some(m);
            }
        } else {
            unsafe {
                let ptr = &self.failed_material as *const _;
                let ptr = ptr as *mut HashSet<String>;

                (*ptr).insert(String::from(name));
            }
        }

        None
    }

    fn load_material(
        &self,
        name: &str,
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_manager: &ShaderManager,
    ) -> Option<DruvisMaterial> {
        for path in self.search_list.iter() {
            let filename = path.join(String::from(name) + ".json");

            let contents = std::fs::read_to_string(filename);
            if contents.is_ok() {
                let desc: MaterialDescriptor = serde_json::from_str(contents.as_ref().unwrap()).unwrap();
                let mat = DruvisMaterial::from_descriptor(device, builtin_bind_group_layouts, shader_manager, &desc)?;

                return Some(mat);
            }
        }

        None
    }
}