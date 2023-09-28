use std::{collections::HashMap, any::{TypeId, Any}, rc::Rc, cell::RefCell};

use super::{component::DruvisComponent, transform::TransformComponentData};

pub struct DruvisGameObject {
    pub components: HashMap<TypeId, Rc<dyn Any>>,
}

impl DruvisGameObject {
    pub fn new() -> Rc<RefCell<Self>> {
        let transform: Rc<dyn Any> = Rc::new(RefCell::new(DruvisComponent::<TransformComponentData>::default()));
        let mut components = HashMap::new();
        components.insert(TypeId::of::<TransformComponentData>(), transform.clone());

        let go = Rc::new(RefCell::new(Self {
            components
        }));

        let trans = Rc::downcast::<RefCell<DruvisComponent<TransformComponentData>>>(transform).unwrap();
        trans.borrow_mut().game_object = Some(Rc::downgrade(&go));

        go
    }

    pub fn add_component<T: Any>(this: Rc<RefCell<DruvisGameObject>>, component: DruvisComponent<T>) -> bool {
        let type_id = TypeId::of::<T>();
        if this.borrow().components.contains_key(&type_id) {
            return false;
        }

        let mut component = component;
        component.game_object = Some(Rc::downgrade(&this));

        this.borrow_mut().components.insert(type_id, Rc::new(RefCell::new(component)));
        true
    }

    pub fn get_component<T: Any>(this: Rc<RefCell<DruvisGameObject>>) -> Option<Rc<RefCell<DruvisComponent<T>>>> {
        let type_id = TypeId::of::<T>();
        if !this.borrow().components.contains_key(&type_id) {
            return None;
        }

        let component = this.borrow().components.get(&type_id).unwrap().clone();
        let concrete = Rc::downcast::<RefCell<DruvisComponent<T>>>(component).unwrap();
        Some(concrete)
    }

    pub fn has_component<T: Any>(this: Rc<RefCell<DruvisGameObject>>) -> bool {
        DruvisGameObject::get_component::<T>(this).is_some()
    }

    pub fn remove_component<T: Any>(this: Rc<RefCell<DruvisGameObject>>) -> bool {
        let type_id = TypeId::of::<T>();
        if !this.borrow().components.contains_key(&type_id) {
            return false;
        }

        this.borrow_mut().components.remove(&type_id);
        true
    }
}

pub trait DruvisGameObjectExt {
    fn add_component<T: Any>(&self, component: DruvisComponent<T>) -> bool;
}

impl DruvisGameObjectExt for Rc<RefCell<DruvisGameObject>> {
    fn add_component<T: Any>(&self, component: DruvisComponent<T>) -> bool {
        DruvisGameObject::add_component(self.clone(), component)
    }
}
