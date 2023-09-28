pub mod game_object;
pub mod component;
pub mod transform;
pub mod components;

pub use game_object::DruvisGameObject;
pub use component::DruvisComponent;
pub use transform::TransformComponentData;

use self::components::MeshRendererData;

pub type MeshRenderer = DruvisComponent<MeshRendererData>;
pub type Transform = DruvisComponent<TransformComponentData>;