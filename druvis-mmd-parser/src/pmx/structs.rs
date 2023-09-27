use anyhow::Result;
use crate::utils;

type byte = i8;
type ubyte = u8;
type short = i16;
type ushort = u16;
type int = i32;
type uint = u32;
type float = f32;
type text = (i32, Vec<u8>);

pub struct PMXHeaderRaw {
    pub signature: [i8; 4],
    pub version: f32,
    pub globals_count: i8,
    pub globals: Vec<i8>,
    pub model_name_local: text,
    pub model_name_universal: text,
    pub comments_local: text,
    pub comments_universal: text,
}

impl PMXHeaderRaw {
    pub fn new() -> Self {
        PMXHeaderRaw {
            signature: [0; 4],
            version: 2.0,
            globals_count: 8,
            globals: Vec::new(),
            model_name_local: (0, Vec::new()),
            model_name_universal: (0, Vec::new()),
            comments_local: (0, Vec::new()),
            comments_universal: (0, Vec::new())
        }
    }

    pub fn to_globals_raw(&self) -> PMXGlobalsRaw {
        let mut result = PMXGlobalsRaw::new();
        let mut temp = [
            &mut result.text_encoding,
            &mut result.additional_vec4_count,
            &mut result.vertex_index_size,
            &mut result.texture_index_size,
            &mut result.material_index_size,
            &mut result.bone_index_size,
            &mut result.morph_index_size,
            &mut result.rigidbody_index_size
        ];
        for i in 0..self.globals_count as usize {
            *temp[i] = Some(self.globals[i]);
        }

        result
    }
}

#[derive(Debug)]
pub struct PMXHeader {
    pub signature: [i8; 4],
    pub version: f32,
    pub globals_count: i8,
    pub globals: Vec<i8>,
    pub model_name_local: String,
    pub model_name_universal: String,
    pub comments_local: String,
    pub comments_universal: String,
}

impl PMXHeader {
    pub fn from_pmx_header_raw(raw: &PMXHeaderRaw, globals: &PMXGlobals) -> Result<Self> {
        let result = Self {
            signature: raw.signature.clone(),
            version: raw.version,
            globals_count: raw.globals_count,
            globals: raw.globals.clone(),
            model_name_local: globals.text_encoding.parse_text(&raw.model_name_local)?,
            model_name_universal: globals.text_encoding.parse_text(&raw.model_name_universal)?,
            comments_local: globals.text_encoding.parse_text(&raw.comments_local)?,
            comments_universal: globals.text_encoding.parse_text(&raw.comments_universal)?,
        };
        Ok(result)
    }
}

pub struct PMXGlobalsRaw {
    pub text_encoding: Option<i8>,
    pub additional_vec4_count: Option<i8>,
    pub vertex_index_size: Option<i8>,
    pub texture_index_size: Option<i8>,
    pub material_index_size: Option<i8>,
    pub bone_index_size: Option<i8>,
    pub morph_index_size: Option<i8>,
    pub rigidbody_index_size: Option<i8>,
}

impl PMXGlobalsRaw {
    pub fn new() -> Self {
        PMXGlobalsRaw {
            text_encoding: None,
            additional_vec4_count: None,
            vertex_index_size: None,
            texture_index_size: None,
            material_index_size: None,
            bone_index_size: None,
            morph_index_size: None,
            rigidbody_index_size: None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextEncodingType {
    UTF16LE,
    UTF8
}

impl TextEncodingType {
    pub fn parse_text(&self, raw: &(i32, Vec<u8>)) -> anyhow::Result<String> {
        let slice = unsafe {
            let ptr = raw.1.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, raw.1.len())
        };

        let s = match *self {
            // todo
            TextEncodingType::UTF16LE => unsafe {
                let ptr = raw.1.as_ptr() as *const u16;
                let slice = std::slice::from_raw_parts(ptr, raw.1.len() / 2);
                String::from_utf16(slice)?
            },
            TextEncodingType::UTF8 => String::from(std::str::from_utf8(slice)?),
        };

        Ok(String::from(s))
    }
}

impl From<i8> for TextEncodingType {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::UTF16LE,
            1 => Self::UTF8,
            _ => panic!("invalid PMX text encoding {}", value)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PMXIndexType {
    B1,
    B2,
    B4
}

impl PMXIndexType {
    pub fn to_usize(&self) -> usize {
        match *self {
            Self::B1 => 1,
            Self::B2 => 2,
            Self::B4 => 4
        }
    }

    pub fn parse_i32(&self, data: &[u8], cursor: &mut usize, is_vertex: bool) -> i32 {
        if is_vertex {
            match *self {
                Self::B1 => utils::read::<u8>(data, cursor) as i32,
                Self::B2 => utils::read::<u16>(data, cursor) as i32,
                Self::B4 => utils::read::<i32>(data, cursor)
            }
        } else {
            match *self {
                Self::B1 => utils::read::<i8>(data, cursor) as i32,
                Self::B2 => utils::read::<i16>(data, cursor) as i32,
                Self::B4 => utils::read::<i32>(data, cursor)
            }
        }
    }
}

impl From<i8> for PMXIndexType {
    fn from(value: i8) -> Self {
        match value {
            1 => Self::B1,
            2 => Self::B2,
            4 => Self::B4,
            _ => panic!("invalid PMX index type {}", value)
        }
    }
}

#[derive(Debug)]
pub struct PMXGlobals {
    pub text_encoding: TextEncodingType,
    pub additional_vec4_count: i8,
    pub vertex_index_size: PMXIndexType,
    pub texture_index_size: PMXIndexType,
    pub material_index_size: PMXIndexType,
    pub bone_index_size: PMXIndexType,
    pub morph_index_size: PMXIndexType,
    pub rigidbody_index_size: PMXIndexType
}

impl From<&PMXGlobalsRaw> for PMXGlobals {
    fn from(value: &PMXGlobalsRaw) -> Self {
        Self {
            text_encoding: value.text_encoding.unwrap_or(1).into(),
            additional_vec4_count: value.additional_vec4_count.unwrap_or(0),
            vertex_index_size: value.vertex_index_size.unwrap_or(4).into(),
            texture_index_size: value.texture_index_size.unwrap_or(4).into(),
            material_index_size: value.material_index_size.unwrap_or(4).into(),
            bone_index_size: value.bone_index_size.unwrap_or(4).into(),
            morph_index_size: value.morph_index_size.unwrap_or(4).into(),
            rigidbody_index_size: value.rigidbody_index_size.unwrap_or(4).into()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PMXWeightDeformType {
    BDEF1,
    BDEF2,
    BDEF4,
    SDEF,
    QDEF,
}

impl PMXWeightDeformType {
    pub fn parse(data: &[u8], cursor: &mut usize) -> Self {
        let ty = utils::read::<i8>(data, cursor);
        match ty {
            0 => Self::BDEF1,
            1 => Self::BDEF2,
            2 => Self::BDEF4,
            3 => Self::SDEF,
            4 => Self::QDEF,
            _ => panic!("Undefined Deform Type {}", ty)
        }
    }
}

pub struct BDEF1Data {
    pub bone_index: Vec<u8>,
}

impl BDEF1Data {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize) -> Self {
        Self {
            bone_index: utils::read_var::<u8>(data, cursor, index_size)
        }
    }
}

pub struct BDEF2Data {
    pub bone_index1: Vec<u8>,
    pub bone_index2: Vec<u8>,
    pub bone1_weight: f32,
}

impl BDEF2Data {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize) -> Self {
        Self {
            bone_index1: utils::read_var::<u8>(data, cursor, index_size),
            bone_index2: utils::read_var::<u8>(data, cursor, index_size),
            bone1_weight: utils::read::<f32>(data, cursor)
        }
    }
}

pub struct BDEF4Data {
    pub bone_index1: Vec<u8>,
    pub bone_index2: Vec<u8>,
    pub bone_index3: Vec<u8>,
    pub bone_index4: Vec<u8>,
    pub bone1_weight: f32,
    pub bone2_weight: f32,
    pub bone3_weight: f32,
    pub bone4_weight: f32,
}

impl BDEF4Data {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize) -> Self {
        Self {
            bone_index1: utils::read_var::<u8>(data, cursor, index_size),
            bone_index2: utils::read_var::<u8>(data, cursor, index_size),
            bone_index3: utils::read_var::<u8>(data, cursor, index_size),
            bone_index4: utils::read_var::<u8>(data, cursor, index_size),
            bone1_weight: utils::read::<f32>(data, cursor),
            bone2_weight: utils::read::<f32>(data, cursor),
            bone3_weight: utils::read::<f32>(data, cursor),
            bone4_weight: utils::read::<f32>(data, cursor),
        }
    }
}

pub struct SDEFData {
    pub bone_index1: Vec<u8>,
    pub bone_index2: Vec<u8>,
    pub bone1_weight: f32,
    pub c: [f32; 3],
    pub r0: [f32; 3],
    pub r1: [f32; 3],
}

impl SDEFData {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize) -> Self {
        Self {
            bone_index1: utils::read_var::<u8>(data, cursor, index_size),
            bone_index2: utils::read_var::<u8>(data, cursor, index_size),
            bone1_weight: utils::read::<f32>(data, cursor),
            c: utils::read::<[f32; 3]>(data, cursor),
            r0: utils::read::<[f32; 3]>(data, cursor),
            r1: utils::read::<[f32; 3]>(data, cursor),
        }
    }
}

pub struct QDEFData {
    pub bone_index1: Vec<u8>,
    pub bone_index2: Vec<u8>,
    pub bone_index3: Vec<u8>,
    pub bone_index4: Vec<u8>,
    pub bone1_weight: f32,
    pub bone2_weight: f32,
    pub bone3_weight: f32,
    pub bone4_weight: f32,
}

impl QDEFData {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize) -> Self {
        Self {
            bone_index1: utils::read_var::<u8>(data, cursor, index_size),
            bone_index2: utils::read_var::<u8>(data, cursor, index_size),
            bone_index3: utils::read_var::<u8>(data, cursor, index_size),
            bone_index4: utils::read_var::<u8>(data, cursor, index_size),
            bone1_weight: utils::read::<f32>(data, cursor),
            bone2_weight: utils::read::<f32>(data, cursor),
            bone3_weight: utils::read::<f32>(data, cursor),
            bone4_weight: utils::read::<f32>(data, cursor),
        }
    }
}

pub enum PMXWeightDeformData {
    BDEF1(BDEF1Data),
    BDEF2(BDEF2Data),
    BDEF4(BDEF4Data),
    SDEF(SDEFData),
    QDEF(QDEFData),
}

impl PMXWeightDeformData {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize, ty: PMXWeightDeformType) -> Self {
        match ty {
            PMXWeightDeformType::BDEF1 => Self::BDEF1(BDEF1Data::parse(data, cursor, index_size)),
            PMXWeightDeformType::BDEF2 => Self::BDEF2(BDEF2Data::parse(data, cursor, index_size)),
            PMXWeightDeformType::BDEF4 => Self::BDEF4(BDEF4Data::parse(data, cursor, index_size)),
            PMXWeightDeformType::SDEF => Self::SDEF(SDEFData::parse(data, cursor, index_size)),
            PMXWeightDeformType::QDEF => Self::QDEF(QDEFData::parse(data, cursor, index_size)),
        }
    }
}

pub struct PMXVertexData {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub additional_vec4: Vec<[f32; 4]>,
    pub weight_deform_type: PMXWeightDeformType,
    pub weight_deform: PMXWeightDeformData,
    pub edge_scale: f32,
}

impl PMXVertexData {
    pub fn parse(data: &[u8], cursor: &mut usize, index_size: usize, addition_vec4_size: usize) -> Self {
        let position = utils::read::<[f32; 3]>(data, cursor);
        let normal = utils::read::<[f32; 3]>(data, cursor);
        let uv = utils::read::<[f32; 2]>(data, cursor);
        let additional_vec4 = utils::read_var::<[f32; 4]>(data, cursor, addition_vec4_size);
        let weight_deform_type = PMXWeightDeformType::parse(data, cursor);
        let weight_deform = PMXWeightDeformData::parse(data, cursor, index_size, weight_deform_type);
        let edge_scale = utils::read::<f32>(data, cursor);

        // println!("{:?}", position);
        
        Self {
            position,
            normal,
            uv,
            additional_vec4,
            weight_deform_type,
            weight_deform,
            edge_scale
        }
    }
}

pub struct PMXSurfaceData {
    // for convenience, use i32 to represent a index type
    pub triangle: [i32; 3],
}

impl PMXSurfaceData {
    pub fn parse(data: &[u8], cursor: &mut usize, vertex_index_size: PMXIndexType) -> Self {
        Self {
            triangle: [
                vertex_index_size.parse_i32(data, cursor, true),
                vertex_index_size.parse_i32(data, cursor, true),
                vertex_index_size.parse_i32(data, cursor, true),
            ]
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMXEnvironmentBlendMode {
    Disabled,
    Multiply,
    Additive,
    AdditionalVec4,
}

impl PMXEnvironmentBlendMode {
    pub fn parse(data: &[u8], cursor: &mut usize) -> Self {
        let ty = utils::read::<i8>(data, cursor);
        match ty {
            0 => Self::Disabled,
            1 => Self::Multiply,
            2 => Self::Additive,
            3 => Self::AdditionalVec4,
            _ => panic!("invalid environment blend mode")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMXToonReference {
    Texture,
    Internal
}

impl PMXToonReference {
    pub fn parse(data: &[u8], cursor: &mut usize) -> Self {
        let ty = utils::read::<i8>(data, cursor);
        match ty {
            0 => Self::Texture,
            1 => Self::Internal,
            _ => panic!("invalid toon reference type")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMXToonValue {
    Texture(i32),
    Internal(i8),
}

impl PMXToonValue {
    pub fn parse(data: &[u8], cursor: &mut usize, ty: PMXToonReference, texture_index_size: PMXIndexType) -> Self {
        match ty {
            PMXToonReference::Texture => Self::Texture(
                texture_index_size.parse_i32(data, cursor, false)
            ),
            PMXToonReference::Internal => Self::Internal(
                utils::read::<i8>(data, cursor)
            )
        }
    }
}

pub struct PMXMaterialData {
    pub material_name_local: String,
    pub material_name_universal: String,
    pub diffuse_color: [f32; 4],
    pub specular_color: [f32; 3],
    pub specular_strength: f32,
    pub ambient_color: [f32; 3],
    pub drawing_flags: u8,
    pub edge_color: [f32; 4],
    pub edge_scale: f32,
    pub texture_index: i32,
    pub environment_index: i32,
    pub environment_blend_mode: PMXEnvironmentBlendMode,
    pub toon_reference: PMXToonReference,
    pub toon_value: PMXToonValue,
    pub meta_data: String,
    pub surface_count: i32,
}

impl PMXMaterialData {
    pub fn parse(data: &[u8], cursor: &mut usize, texture_index_size: PMXIndexType, text_encoding: TextEncodingType) -> Result<Self> {
        let material_name_local = utils::read_text(data, cursor);
        let material_name_universal = utils::read_text(data, cursor);
        let diffuse_color = utils::read::<[f32; 4]>(data, cursor);
        let specular_color = utils::read::<[f32; 3]>(data, cursor);
        let specular_strength = utils::read::<f32>(data, cursor);
        let ambient_color = utils::read::<[f32; 3]>(data, cursor);
        let drawing_flags = utils::read::<u8>(data, cursor);
        let edge_color = utils::read::<[f32; 4]>(data, cursor);
        let edge_scale = utils::read::<f32>(data, cursor);
        let texture_index = texture_index_size.parse_i32(data, cursor, false);
        let environment_index = texture_index_size.parse_i32(data, cursor, false);
        let environment_blend_mode = PMXEnvironmentBlendMode::parse(data, cursor);
        let toon_reference = PMXToonReference::parse(data, cursor);
        let toon_value = PMXToonValue::parse(data, cursor, toon_reference, texture_index_size);
        let meta_data = utils::read_text(data, cursor);
        let surface_count = utils::read::<i32>(data, cursor);

        Ok(Self {
            material_name_local: text_encoding.parse_text(&material_name_local)?,
            material_name_universal: text_encoding.parse_text(&material_name_universal)?,
            diffuse_color,
            specular_color,
            specular_strength,
            ambient_color,
            drawing_flags,
            edge_color,
            edge_scale,
            texture_index,
            environment_index,
            environment_blend_mode,
            toon_reference,
            toon_value,
            meta_data: text_encoding.parse_text(&meta_data)?,
            surface_count
        })
    }
}