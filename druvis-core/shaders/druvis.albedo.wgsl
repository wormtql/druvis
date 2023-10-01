// align = 16
struct CameraUniform {
    druvis_world_space_camera_position: vec4<f32>,
    druvis_view_matrix: mat4x4<f32>,
    druvis_projection_matrix: mat4x4<f32>,
    druvis_projection_params: vec4<f32>,
};

// align = 16
struct LightUniform {
    druvis_light_type: u32,
    druvis_light_intensity: f32,
    druvis_light_color: vec4<f32>,
    druvis_light_position: vec4<f32>,
    druvis_light_direction: vec4<f32>,
};

struct PerFrameUniform {
    camera_uniform: CameraUniform,
    light_uniform: LightUniform,
}

@group(0) @binding(0)
var<uniform> per_frame_uniform: PerFrameUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    // @location(3) tangent: vec3<f32>,
    // @location(4) bitangent: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
};

// struct ShaderProperties {
    // color: vec4<f32>,
// };
// @group(2) @binding(0)
// var<uniform> shader_properties: ShaderProperties;

// albedo texture
@group(2) @binding(1)
var albedo_texture: texture_2d<f32>;
@group(2) @binding(2)
var albedo_texture_sampler: sampler;

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    // var out: VertexOutput;
    // let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    // out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    // out.color = model.position;
    // // out.clip_position = vec4<f32>()
    // return out;

    let projection_matrix: mat4x4<f32> = per_frame_uniform.camera_uniform.druvis_projection_matrix;
    let view_matrix: mat4x4<f32> = per_frame_uniform.camera_uniform.druvis_view_matrix;

    let world_pos = vec4<f32>(model.position, 1.0);

    var out: VertexOutput;
    out.clip_position = projection_matrix * view_matrix * world_pos;
    // out.clip_position = camera.druvis_projection_matrix * vec4<f32>(model.position, 1.0);
    // out.clip_position = vec4<f32>(model.position.xy, 0.5, 1.0);
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    out.world_pos = world_pos.xyz;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(-per_frame_uniform.light_uniform.druvis_light_direction.xyz);
    let light_color = per_frame_uniform.light_uniform.druvis_light_color;
    let light_intensity = per_frame_uniform.light_uniform.druvis_light_intensity;

    let camera_pos: vec3<f32> = per_frame_uniform.camera_uniform.druvis_world_space_camera_position.xyz;

    let normal = normalize(in.normal);

    let view_dir: vec3<f32> = normalize(camera_pos - in.world_pos);

    var l = dot(normal, light_dir) * 0.5 + 0.5;
    l = l * l;

    // let l = max(dot(in.normal, light_dir), 0.0);

    // return vec4<f32>(view_dir * 0.5 + 0.5, 1.0);
    // return vec4<f32>(light_dir * 0.5 + 0.5, 1.0);
    // return vec4<f32>(in.world_pos, 1.0);
    // return vec4<f32>(camera_pos, 1.0);
    return vec4<f32>(l, l, l, 1.0);

    // return vec4<f32>(light_intensity, light_intensity, light_intensity, 1.0);
    // return vec4<f32>(light_color.xyz, 1.0);

    // let color: vec4<f32> = textureSample(albedo_texture, albedo_texture_sampler, in.tex_coords);
    // return vec4<f32>(color);

    // return vec4<f32>(in.normal * 0.5 + 0.5, 1.0);
    // return vec4<f32>(0.5, 0.6, 0.0, 1.0);
    // return vec4<f32>(in., 1.0);

    // return camera.druvis_projection_matrix[3];
    // return camera.druvis_view_matrix[2];
    // return shader_properties.color;
    // return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    // return vec4<f32>(in.color, 1.0);
}
