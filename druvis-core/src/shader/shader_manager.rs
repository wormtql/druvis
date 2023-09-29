use std::{collections::{HashMap, HashSet}, rc::Rc, path::{PathBuf, Path}, cell::RefCell};
use anyhow::Result;

use super::{shader::DruvisShader, shader_descriptor::{self, ShaderDescriptor}};

pub struct ShaderManager {
    loaded_shaders: RefCell<HashMap<String, Rc<DruvisShader>>>,
    failed_shaders: RefCell<HashSet<String>>,
    search_paths: Vec<PathBuf>,
}

impl ShaderManager {
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    fn load_shader(
        &self,
        path: PathBuf,
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Result<()> {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_name = String::from(file_name);
        println!("file_name: {}", file_name);
        let temp = file_name.split(".").collect::<Vec<_>>();
        let size = temp.len();
        let mut name = String::new();
        for comp in temp.into_iter().rev().skip(1).rev().enumerate() {
            name += comp.1;
            if comp.0 != size - 2 {
                name += "."
            }
        }

        let wgsl = path.parent().unwrap().join(String::from(name) + ".wgsl");
        println!("wgsl: {:?}", wgsl);
        
        let meta = std::fs::read_to_string(path)?;
        let source = std::fs::read_to_string(wgsl)?;
        // println!("source: {}", source);
        // println!("meta: {}", meta);

        let mut shader_desc = serde_json::from_str::<ShaderDescriptor>(&meta).unwrap();
        shader_desc.source = source;

        let shader = DruvisShader::from_shader_descriptor(device, builtin_bind_group_layouts, &shader_desc);

        self.loaded_shaders.borrow_mut().insert(shader_desc.name.clone(), Rc::new(shader));

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            loaded_shaders: RefCell::new(HashMap::new()),
            failed_shaders: RefCell::new(HashSet::new()),
            search_paths: Vec::new(),
        }
    }

    pub fn get_shader(
        &self,
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        name: &str
    ) -> Option<Rc<DruvisShader>> {
        if self.loaded_shaders.borrow().contains_key(name) {
            return Some(self.loaded_shaders.borrow().get(name).unwrap().clone())
        }
        if self.failed_shaders.borrow().contains(name) {
            return None;
        }

        for path in self.search_paths.iter() {
            let meta = path.join(String::from(name) + ".json");
            let result = self.load_shader(meta, device, builtin_bind_group_layouts);
            if result.is_ok() {
                return Some(self.loaded_shaders.borrow().get(name).unwrap().clone());
            }
        }

        self.failed_shaders.borrow_mut().insert(String::from(name));

        None
    }
}
