use crate::types::RcRefCell;
use super::slots::Slots;
use super::heap::class::Class;

pub struct Object {
    class: RcRefCell<Class>,
    fields: Slots,
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
    pub fn new(class: RcRefCell<Class>) -> Self {
        Object {
            class: class.clone(),
            fields: Slots::new(class.borrow().instance_slot_count() as usize),
        }
    }

    pub fn class(&self) -> &RcRefCell<Class> {
        &self.class
    }

    pub fn fields(&self) -> &Slots {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut Slots {
        &mut self.fields
    }

    pub fn is_instance_of(&self, class: &RcRefCell<Class>) -> bool {
        class.borrow().is_assignable_from(class, &self.class)
    }
}
