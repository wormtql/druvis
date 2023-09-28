use std::{rc::Rc, cell::RefCell, any::Any};

use crate::game_object::{game_object::DruvisGameObject, DruvisComponent};

pub struct DruvisScene {
    pub objects: Vec<Rc<RefCell<DruvisGameObject>>>,
}

impl DruvisScene {
    pub fn get_components<T: Any>(&self) -> Vec<Rc<RefCell<DruvisComponent<T>>>> {
        let mut result = Vec::new();
        for item in self.objects.iter() {
            let component = DruvisGameObject::get_component::<T>(item.clone());
            if component.is_some() {
                result.push(component.as_ref().unwrap().clone());
            }
        }

        result
    }
}
