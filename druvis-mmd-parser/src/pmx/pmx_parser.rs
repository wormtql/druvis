use std::{mem, collections::HashMap, path::PathBuf, rc::Rc, cell::RefCell};
use anyhow::Result;
use druvis_core::{mesh::mesh::DruvisMesh, vertex::vertex::ModelVertex, material::material::DruvisMaterial, texture::texture::DruvisTextureAndSampler, shader::shader_manager::ShaderManager, game_object::{DruvisGameObject, DruvisComponent, components::MeshRendererData, game_object::DruvisGameObjectExt}};
use crate::{utils, pmx::structs::{PMXVertexData, PMXMaterialData}};

use super::structs::{PMXHeaderRaw, PMXGlobalsRaw, PMXGlobals, PMXHeader, PMXSurfaceData};

#[derive(Clone, Debug)]
pub struct PMXFormat {
    pub header: PMXHeader,
    pub globals: PMXGlobals,
    pub vertices: Vec<PMXVertexData>,
    pub surfaces: Vec<PMXSurfaceData>,
    pub texture_paths: Vec<String>,
    pub materials: Vec<PMXMaterialData>,

    model_path: PathBuf,
}

impl PMXFormat {
    pub fn create_game_object(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shader_manager: &ShaderManager,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Rc<RefCell<DruvisGameObject>> {
        println!("vertex count: {}", self.vertices.len());
        println!("index count: {}", self.surfaces.len() * 3);
        // println!("mat {}", self.materials[self.materials.len() - 1].)

        let mut go = DruvisGameObject::new();

        let mesh = self.clone().to_druvis_mesh(device);
        let mut mesh_renderer = DruvisComponent::<MeshRendererData>::default();
        mesh_renderer.data.mesh = Some(Rc::new(RefCell::new(mesh)));

        let mut mats = Vec::new();
        let material_count = self.materials.len();
        for i in 0..material_count {
            let mat = self.create_material(device, queue, i, shader_manager, builtin_bind_group_layouts);
            mats.push(Rc::new(RefCell::new(mat.unwrap())));
        }
        mesh_renderer.data.materials = mats;

        go.add_component(mesh_renderer);

        go
    }

    pub fn create_material(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mat_index: usize,
        shader_manager: &ShaderManager,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Option<DruvisMaterial> {
        let mat = &self.materials[mat_index];

        let mut textures = HashMap::new();
        let diffuse_texture_path = self.model_path.join(&self.texture_paths[mat.texture_index as usize]);
        let diffuse_texture = DruvisTextureAndSampler::from_path(
            device,
            queue,
            &diffuse_texture_path,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            },
            wgpu::SamplerBindingType::Filtering
        );
        textures.insert(String::from("albedo_texture"), Rc::new(diffuse_texture));

        let shader = shader_manager.get_shader(device, builtin_bind_group_layouts, "druvis.albedo")?;
        let druvis_mat = DruvisMaterial::create_material(
            device,
            shader,
            textures,
            "mmd_mat"
        );

        druvis_mat
    }

    pub fn to_druvis_mesh(self, device: &wgpu::Device) -> DruvisMesh {
        let mut vertices: Vec<ModelVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut submeshes: Vec<(u64, u64)> = Vec::new();

        for v in self.vertices.iter() {
            let model_vertex = ModelVertex {
                position: v.position.clone(),
                tex_coords: v.uv.clone(),
                normal: v.normal.clone(),
                // todo calculate tangents
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0]
            };
            vertices.push(model_vertex);
        }
        for surface in self.surfaces.iter() {
            indices.push(surface.triangle[0] as u32);
            indices.push(surface.triangle[1] as u32);
            indices.push(surface.triangle[2] as u32);
        }

        let mut start: u64 = 0;
        for mat in self.materials.iter() {
            // println!("start {}, {}, range {}", start, start + mat.surface_count as u64, mat.surface_count);
            submeshes.push((start, start + mat.surface_count as u64));
            start += mat.surface_count as u64;
        }

        DruvisMesh::new(
            device,
            &self.header.model_name_local,
            vertices,
            indices,
            submeshes
        )
    }
}

pub struct PmxParser {

}

impl PmxParser {
    pub fn new() -> Self {
        PmxParser {  }
    }
}

impl PmxParser {
    fn parse_header(&self, data: &[u8], cursor: &mut usize) -> Result<PMXHeaderRaw> {
        let mut result = PMXHeaderRaw::new();
        result.signature = utils::read::<[i8; 4]>(data, cursor);
        result.version = utils::read::<f32>(data, cursor);
        result.globals_count = utils::read::<i8>(data, cursor);
        result.globals = utils::read_var::<i8>(data, cursor, result.globals_count as usize);
        result.model_name_local = utils::read_text(data, cursor);
        result.model_name_universal = utils::read_text(data, cursor);
        result.comments_local = utils::read_text(data, cursor);
        result.comments_universal = utils::read_text(data, cursor);
        // println!("signature: {:?}", result.signature);
        // println!("version: {:?}", result.version);
        // println!("globals_count: {:?}", result.globals_count);
        // println!("globals: {:?}", result.globals);
        // println!("model_name_local: {:?}", result.model_name_local);
        // println!("model_name_universal: {:?}", result.model_name_universal);
        // println!("comments_local: {:?}", result.comments_local);
        // println!("comments_universal: {:?}", result.comments_universal);
        Ok(result)
    }

    pub fn parse(&self, data: &[u8], model_path: PathBuf) -> Result<PMXFormat> {
        let mut cursor: usize = 0;

        let header_raw = self.parse_header(data, &mut cursor)?;

        let global = PMXGlobals::from(&header_raw.to_globals_raw());
        let header = PMXHeader::from_pmx_header_raw(&header_raw, &global)?;
        println!("{:?}", header);

        let vertex_count = utils::read::<i32>(data, &mut cursor);
        let mut vertices: Vec<PMXVertexData> = Vec::new();
        println!("vertex count: {}", vertex_count);
        for _ in 0..vertex_count {
            vertices.push(PMXVertexData::parse(
                data,
                &mut cursor,
                global.bone_index_size.to_usize(),
                global.additional_vec4_count as usize
            ));
        }

        let surface_count = utils::read::<i32>(data, &mut cursor) / 3;
        let mut surfaces = Vec::new();
        println!("face count: {}", surface_count);
        for _ in 0..surface_count {
            surfaces.push(PMXSurfaceData::parse(
                data,
                &mut cursor,
                global.vertex_index_size
            ))
        }

        // parse texture paths
        let texture_path_count = utils::read::<i32>(data, &mut cursor);
        let mut texture_paths = Vec::new();
        println!("texture path count: {}", texture_path_count);
        for _ in 0..texture_path_count {
            let t = utils::read_text(data, &mut cursor);
            let s = global.text_encoding.parse_text(&t)?;
            println!("{}", s);
            texture_paths.push(s);
        }

        // materials
        let material_count = utils::read::<i32>(data, &mut cursor);
        let mut materials = Vec::new();
        println!("material count: {}", material_count);
        for _ in 0..material_count {
            materials.push(PMXMaterialData::parse(
                data,
                &mut cursor,
                global.texture_index_size,
                global.text_encoding
            )?);
        }

        Ok(PMXFormat {
            header,
            globals: global,
            vertices,
            surfaces,
            texture_paths,
            materials,

            model_path,
        })
    }
}
