use std::mem;
use anyhow::Result;
use druvis_core::{mesh::mesh::DruvisMesh, vertex::vertex::ModelVertex};
use crate::{utils, pmx::structs::{PMXVertexData, PMXMaterialData}};

use super::structs::{PMXHeaderRaw, PMXGlobalsRaw, PMXGlobals, PMXHeader, PMXSurfaceData};

pub struct PMXFormat {
    pub header: PMXHeader,
    pub globals: PMXGlobals,
    pub vertices: Vec<PMXVertexData>,
    pub surfaces: Vec<PMXSurfaceData>,
    pub texture_paths: Vec<String>,
    pub materials: Vec<PMXMaterialData>,
}

impl PMXFormat {
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
            submeshes.push((start, mat.surface_count as u64));
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

    pub fn parse(&self, data: &[u8]) -> Result<PMXFormat> {
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
            materials
        })
    }
}
