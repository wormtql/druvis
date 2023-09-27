use crate::vertex::vertex::{ModelVertex, Vertex};

pub struct ShaderBindState {
    // value is updated per material
    pub value_bind_group_layout: wgpu::BindGroupLayout,
    pub value_bind_group: wgpu::BindGroup,
    pub value_buffer: wgpu::Buffer,

    pub texture_bind_group_layout: Option<wgpu::BindGroupLayout>,
}

pub struct DruvisShader<'a> {
    pub name: String,
    pub shader_module: wgpu::ShaderModule,
    pub render_pipeline_layout: wgpu::PipelineLayout,
    pub render_pipeline: wgpu::RenderPipeline,
    pub cull_mode: wgpu::Face,
    pub blend_state: Option<wgpu::BlendState>,
    pub is_instancing: bool,
    pub instancing_vertex_buffer_layout: Option<wgpu::VertexBufferLayout<'a>>,
    
    pub shader_bind_state: ShaderBindState,
}

impl<'a> DruvisShader<'a> {
    pub fn use_shader<'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(10, &self.shader_bind_state.value_bind_group, &[]);
    }

    pub fn new(
        device: &wgpu::Device,
        label: &str,
        builtin_bind_group_layouts: &[wgpu::BindGroupLayout],
        shader_module: wgpu::ShaderModule,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        blend_state: Option<wgpu::BlendState>,
        cull_mode: wgpu::Face,
        is_instancing: bool,
        instancing_vertex_buffer_layout: Option<wgpu::VertexBufferLayout<'a>>,
        shader_bind_state: ShaderBindState,
        name: &str,
    ) -> Self {
        let mut bind_group_layouts = Vec::new();
        // add built in bind group layouts, including camera/light/ .. etc
        for item in builtin_bind_group_layouts.iter() {
            bind_group_layouts.push(item);
        }
        // add shader specific bind group layouts
        bind_group_layouts.push(&shader_bind_state.value_bind_group_layout);
        if shader_bind_state.texture_bind_group_layout.is_some() {
            bind_group_layouts.push(shader_bind_state.texture_bind_group_layout.as_ref().unwrap());
        }

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some((String::from(label) + "_pipeline_layout").as_str()),
                bind_group_layouts: &bind_group_layouts[..],
                push_constant_ranges: &[]
            }
        );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some((String::from(label) + "_render_pipeline").as_str()),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[
                        ModelVertex::desc(),
                        // todo instancing
                    ]
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: color_format,
                            blend: blend_state.clone(),
                            write_mask: wgpu::ColorWrites::ALL,
                        })
                    ]
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(cull_mode.clone()),
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

        Self {
            shader_module,
            render_pipeline_layout,
            render_pipeline,
            cull_mode,
            blend_state: blend_state.clone(),
            is_instancing,
            instancing_vertex_buffer_layout: instancing_vertex_buffer_layout.clone(),
            shader_bind_state,
            name: String::from(name),
        }
    }
}

pub trait ShaderPropertyTrait {
    fn get_bind_state(device: &wgpu::Device, label: &str) -> ShaderBindState;
}