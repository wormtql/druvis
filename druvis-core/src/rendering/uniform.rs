use crate::{binding::bind_group_layout_builder::BindGroupLayoutBuilder, camera::camera_uniform::CameraUniform, common::transformation_uniform::TransformationUniform};

#[derive(Default, Debug)]
pub struct PerFrameUniform {
    pub camera_uniform: CameraUniform,
    // todo time, lighting etc
}

impl PerFrameUniform {
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let mut builder = BindGroupLayoutBuilder::new();
        builder.add_buffer_entry(device, 0);
        // builder.add_buffer_entry(device, 1);

        builder.build(device, "per_frame_uniform")
    }
}

#[derive(Default, Debug)]
pub struct PerObjectUniform {
    pub transform: TransformationUniform
}

impl PerObjectUniform {
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let mut builder = BindGroupLayoutBuilder::new();

        builder.add_buffer_entry(device, 0);

        builder.build(device, "per_object_uniform")
    }
}
