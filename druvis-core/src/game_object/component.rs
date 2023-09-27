use std::{rc::Weak, cell::RefCell};

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
