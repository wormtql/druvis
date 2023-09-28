use std::{rc::{Weak, Rc}, cell::RefCell, any::Any};

use super::game_object::DruvisGameObject;

pub struct DruvisComponent<T> {
    pub game_object: Option<Weak<RefCell<DruvisGameObject>>>,
    pub data: T,
}

impl<T: Default> Default for DruvisComponent<T> {
    fn default() -> Self {
        Self {
            game_object: None,
            data: T::default()
        }
    }
}

impl<T> DruvisComponent<T> {
    pub fn get_game_object(&self) -> Option<Rc<RefCell<DruvisGameObject>>> {
        if self.game_object.is_none() {
            return None;
        }

        let go = self.game_object.as_ref().unwrap().upgrade();
        go
    }

    pub fn get_component<U: Any>(&self) -> Option<Rc<RefCell<DruvisComponent<U>>>> {
        let go = self.get_game_object();
        if go.is_none() {
            return None;
        }

        DruvisGameObject::get_component::<U>(go.unwrap())
    }
}
