use std::{rc::Rc, cell::RefCell};

use cgmath::{Vector3, Vector4};

use crate::{game_object::{DruvisComponent, TransformComponentData, DruvisGameObject, game_object::DruvisGameObjectExt}, scene::scene::DruvisScene};

use super::light_uniform::LightUniform;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LightType {
    Point,
    Parallel,
    SpotLight,
    Quad,
}

pub struct Light {
    pub ty: LightType,
    pub intensity: f32,
    pub color: Vector3<f32>,
}

impl DruvisComponent<Light> {
    pub fn get_light_uniform(&self) -> LightUniform {
        let transform = self.get_component::<TransformComponentData>();
        let transform = transform.as_ref().unwrap().clone();

        let direction = transform.borrow().data.get_model_matrix() * Vector4::new(0.0, 0.0, 1.0, 0.0);
        let position = transform.borrow().data.position.clone();

        let mut ret = LightUniform::default();
        ret.light_type = self.data.ty as u32;
        ret.intensity = self.data.intensity;
        ret.color = Vector4::new(self.data.color.x, self.data.color.y, self.data.color.z, 0.0).into();
        ret.position = Vector4::new(position.x, position.y, position.z, 0.0).into();
        ret.direction = direction.into();

        ret
    }
}

impl DruvisScene {
    pub fn get_sun_light(&self) -> Option<Rc<RefCell<DruvisGameObject>>> {
        for item in self.objects.iter() {
            if item.has_component::<Light>() {
                let component = item.get_component::<Light>().unwrap();
                if component.borrow().data.ty == LightType::Parallel {
                    return Some(item.clone());
                }
            }
        }

        None
    }
}
