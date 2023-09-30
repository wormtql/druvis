use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::shader::shader_property::ShaderPropertyValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaterialDescriptor {
    pub name: String,
    pub shader_name: String,
    pub properties: HashMap<String, ShaderPropertyValue>,
    // property name => texture name,
    pub texture_properties: HashMap<String, String>,
}