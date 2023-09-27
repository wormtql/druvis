use crate::{vertex::vertex::{ModelVertex, Vertex}, binding::data_binding_state::DataBindingState};

pub struct DruvisShader<'a, T> {
    pub shader_module: wgpu::ShaderModule,
    pub render_pipeline_layout: wgpu::PipelineLayout,
    pub render_pipeline: wgpu::RenderPipeline,
    pub cull_mode: wgpu::Face,
    pub blend_state: Option<wgpu::BlendState>,
    pub is_instancing: bool,
    pub instancing_vertex_buffer_layout: Option<wgpu::VertexBufferLayout<'a>>,
    pub shader_property_bind_state: DataBindingState<T>
}

impl<'a, T> DruvisShader<'a, T> {
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        shader_module: wgpu::ShaderModule,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        blend_state: Option<wgpu::BlendState>,
        cull_mode: wgpu::Face,
        is_instancing: bool,
        instancing_vertex_buffer_layout: Option<wgpu::VertexBufferLayout<'a>>,
        shader_property_bind_state: DataBindingState<T>
    ) -> Self {
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some((String::from(label) + "_pipeline_layout").as_str()),
                bind_group_layouts: &[
                    camera_bind_group_layout,
                    &shader_property_bind_state.bind_group_layout,
                ],
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
            shader_property_bind_state,
        }
    }
}

pub trait ShaderTrait {
}