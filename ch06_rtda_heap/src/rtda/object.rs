use super::Slot;
use super::heap::class::Class;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Object {
    class: Rc<RefCell<Class>>,
    fields: Vec<Slot>,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self == _other
    }

    fn ne(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self != _other
    }
}

impl Object {
    pub fn new(class: Rc<RefCell<Class>>) -> Self {
        Object {
            class: class.clone(),
            fields: vec![Slot::default(); class.borrow().instance_slot_count() as usize],
        }
    }
    pub fn class(&self) -> &Rc<RefCell<Class>> {
        &self.class
    }

    pub fn fields(&self) -> &Vec<Slot> {
        &self.fields
    }
}
