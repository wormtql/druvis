use std::{collections::HashMap, any::{TypeId, Any}, rc::{Rc, Weak}, cell::RefCell};

use super::{component::DruvisComponent, transform::TransformComponentData};

pub struct DruvisGameObject {
    pub components: HashMap<TypeId, Rc<RefCell<dyn Any>>>,
}

impl DruvisGameObject {
    pub fn new() -> Rc<RefCell<Self>> {
        let transform: Rc<RefCell<dyn Any>> = Rc::new(RefCell::new(DruvisComponent::<TransformComponentData>::default()));
        let mut components = HashMap::new();
        components.insert(TypeId::of::<TransformComponentData>(), transform.clone());

        let go = Rc::new(RefCell::new(Self {
            components
        }));

        let mut binding = transform.borrow_mut();
        let trans = binding.downcast_mut::<DruvisComponent<TransformComponentData>>().unwrap();
        trans.game_object = Some(Rc::downgrade(&go));

        go
    }
}
