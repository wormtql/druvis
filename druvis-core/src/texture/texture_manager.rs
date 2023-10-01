use std::{collections::{HashMap, HashSet}, path::PathBuf};

use super::texture::DruvisTextureAndSampler;

// fn create_black_texture(device: &wgpu::Device) {
    // 
// }

pub struct TextureManager {
    pub loaded_texture: HashMap<String, DruvisTextureAndSampler>,
    pub failed_textures: HashSet<String>,

    pub search_paths: Vec<PathBuf>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            loaded_texture: HashMap::new(),
            failed_textures: HashSet::new(),
            search_paths: Vec::new()
        }
    }

    // fn load_texture(&self, name: &str) -> Option<DruvisTextureAndSampler> {
    //     for path in self.search_paths.iter() {
            
    //     }

    //     None
    // }

    // pub fn load_builtin_texture(&mut self) {

    // }
}
