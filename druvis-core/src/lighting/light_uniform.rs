#[repr(C)]
#[derive(Default, Debug)]
pub struct LightUniform {
    // maybe it's better to use a macro to auto calculate this stuff
    pub light_type: u32,        // offset(0)    align(4)    size(4)
    pub intensity: f32,         // offset(4)    align(4)    size(4)
    pub _padding: [u32; 2],     // offset(8)                size(8)
    pub color: [f32; 4],        // offset(16)   align(16)   size(16)
    pub position: [f32; 4],     // offset(32)   align(16)   size(16)
    pub direction: [f32; 4],    // offset(48)   align(16)   size(16)
}

// impl Default for LightUniform {
//     fn default() -> Self {
//         Self {
//             light_type: LightType::Parallel as u32,
//             _padding: [0; 3],
//             intensity: 0.0,
//             _padding2: [0; 3],
//             color: [0.0; 3],
//             _padding3: 0,
//             position: [0.0, 3],
//             _padding4: 0,
//             direction: [0.0]
//         }
//     }
// }