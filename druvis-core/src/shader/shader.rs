use std::{collections::HashMap, rc::Rc, cell::{RefCell, Ref}};
use serde::{Serialize, Deserialize};

use crate::vertex::vertex::{ModelVertex, Vertex};

use super::{shader_property::{ShaderPropertyLayoutEntry, ShaderTextureLayoutEntry}, shader_descriptor::ShaderDescriptor};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct OwnedVertexBufferLayout {
    /// The stride, in bytes, between elements of this buffer.
    pub array_stride: wgpu::BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: wgpu::VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: Vec<wgpu::VertexAttribute>,
}

pub struct ShaderBindState {
    // value is updated per material
    pub value_bind_group_layout: wgpu::BindGroupLayout,
    // pub value_bind_group: wgpu::BindGroup,
    pub value_buffer: wgpu::Buffer,

    // pub texture_bind_group_layout: Option<wgpu::BindGroupLayout>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct WGPURenderPipelineKey {
    pub color_format: wgpu::TextureFormat,
    pub depth_format: Option<wgpu::TextureFormat>,
}

pub struct WGPURenderPipelineCollection {
    pub data: HashMap<WGPURenderPipelineKey, wgpu::RenderPipeline>,
}

pub struct DruvisShader {
    pub name: String,
    pub shader_module: wgpu::ShaderModule,
    pub render_pipeline_layout: wgpu::PipelineLayout,
    // pub render_pipeline: wgpu::RenderPipeline,
    pub render_pipeline_collection: WGPURenderPipelineCollection,
    pub cull_mode: Option<wgpu::Face>,
    pub blend_state: Option<wgpu::BlendState>,
    pub is_instancing: bool,
    pub instancing_vertex_buffer_layout: Option<OwnedVertexBufferLayout>,
    
    pub shader_value_layout: Vec<ShaderPropertyLayoutEntry>,
    pub shader_texture_layout: Vec<ShaderTextureLayoutEntry>,
    pub shader_value_size: usize,
    pub shader_bind_state: ShaderBindState,
}

impl DruvisShader {
    pub fn use_shader<'a, 'b>(
        &'b self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass<'a>,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) where 'b: 'a {
        let rp = self.get_render_pipeline(device, color_format, depth_format);
        render_pass.set_pipeline(&rp);
        // render_pass.set_bind_group(10, &self.shader_bind_state.value_bind_group, &[]);
    }

    pub fn get_render_pipeline(
        &self,
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>
    ) -> &wgpu::RenderPipeline {
        let key = WGPURenderPipelineKey {
            color_format,
            depth_format
        };
        if self.render_pipeline_collection.data.contains_key(&key) {
            return self.render_pipeline_collection.data.get(&key).unwrap();
            // return ;
        }

        let rp = self.create_render_pipeline(device, color_format, depth_format);

        unsafe {
            let ptr = &self.render_pipeline_collection as *const WGPURenderPipelineCollection;
            let ptr = ptr as *mut WGPURenderPipelineCollection;
            (*ptr).data.insert(key.clone(), rp);
        }

        self.render_pipeline_collection.data.get(&key).unwrap()
    }

    pub fn from_shader_descriptor(
        device: &wgpu::Device,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        desc: &ShaderDescriptor
    ) -> Self {
        let shader_module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some(&desc.name),
                source: wgpu::ShaderSource::Wgsl(desc.source.clone().into())
            }
        );
        DruvisShader::new(
            device,
            &desc.name,
            builtin_bind_group_layouts,
            shader_module,
            desc.blend_mode,
            desc.cull_mode,
            desc.is_instancing,
            desc.instancing_vertex_buffer_layout.clone(),
            desc.shader_value_layout.clone(),
            desc.shader_texture_layout.clone(),
            ShaderBindState {
                value_bind_group_layout: desc.get_bind_group_layout(device),
                value_buffer: desc.get_properties_buffer(device)
            },
            &desc.name
        )
    }

    pub fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>
    ) -> wgpu::RenderPipeline {
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some((self.name.clone() + "_render_pipeline").as_str()),
                layout: Some(&self.render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &self.shader_module,
                    entry_point: "vs_main",
                    buffers: &[
                        ModelVertex::desc(),
                        // todo instancing
                    ]
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.shader_module,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: color_format,
                            blend: self.blend_state,
                            write_mask: wgpu::ColorWrites::ALL,
                        })
                    ]
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: self.cull_mode,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: depth_format.map(|format| {
                    wgpu::DepthStencilState {
                        format,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None
            }
        );

        render_pipeline
    }

    pub fn new(
        device: &wgpu::Device,
        label: &str,
        builtin_bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_module: wgpu::ShaderModule,
        // color_format: wgpu::TextureFormat,
        // depth_format: Option<wgpu::TextureFormat>,
        blend_state: Option<wgpu::BlendState>,
        cull_mode: Option<wgpu::Face>,
        is_instancing: bool,
        instancing_vertex_buffer_layout: Option<OwnedVertexBufferLayout>,
        shader_value_layout: Vec<ShaderPropertyLayoutEntry>,
        shader_texture_layout: Vec<ShaderTextureLayoutEntry>,
        shader_bind_state: ShaderBindState,
        name: &str,
    ) -> Self {
        // shader_bind_state.value_bind_group_layout
        let mut bind_group_layouts = Vec::new();
        // add built in bind group layouts, including camera/light/ .. etc
        for &item in builtin_bind_group_layouts.iter() {
            bind_group_layouts.push(item);
        }
        // add shader specific bind group layouts
        bind_group_layouts.push(&shader_bind_state.value_bind_group_layout);
        // println!("{:?}", bind_group_layouts);
        // if shader_bind_state.texture_bind_group_layout.is_some() {
        //     bind_group_layouts.push(shader_bind_state.texture_bind_group_layout.as_ref().unwrap());
        // }

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some((String::from(label) + "_pipeline_layout").as_str()),
                bind_group_layouts: &bind_group_layouts[..],
                push_constant_ranges: &[]
            }
        );

        let mut value_size = 0;
        for item in shader_value_layout.iter() {
            value_size += item.ty.get_size();
        }

        Self {
            shader_module,
            render_pipeline_layout,
            // render_pipeline,
            render_pipeline_collection: WGPURenderPipelineCollection { data: HashMap::new() },
            cull_mode,
            blend_state: blend_state.clone(),
            is_instancing,
            instancing_vertex_buffer_layout: instancing_vertex_buffer_layout.clone(),
            shader_bind_state,
            shader_value_layout,
            shader_texture_layout,
            shader_value_size: value_size,
            name: String::from(name),
        }
    }
}

pub trait ShaderPropertyTrait {
    fn get_bind_state(device: &wgpu::Device, label: &str) -> ShaderBindState;

    fn get_shader_value_layout() -> Vec<ShaderPropertyLayoutEntry>;

    fn get_shader_texture_layout() -> Vec<ShaderTextureLayoutEntry>;
}
