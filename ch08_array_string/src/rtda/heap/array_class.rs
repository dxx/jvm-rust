use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::rtda::Object;
use super::class::Class;
use super::class_name_helper::get_component_class_name;

impl Class {
    pub fn is_array(&self) -> bool {
        self.name().as_bytes()[0] == b'['
    }

    pub fn new_array(&self, _self: RcRefCell<Class>, count: usize) -> Object {
        if !self.is_array() {
            panic!("Not array class: {}", self.name());
        }

        if self.name() == "[Z" {
            Object::new_data(_self, Box::new(vec![0_i8; count]))
        } else if self.name() == "[B" {
            Object::new_data(_self, Box::new(vec![0_i8; count]))
        } else if self.name() == "[C" {
            Object::new_data(_self, Box::new(vec![0_u16; count]))
        } else if self.name() == "[S" {
            Object::new_data(_self, Box::new(vec![0_i16; count]))
        } else if self.name() == "[I" {
            Object::new_data(_self, Box::new(vec![0_i32; count]))
        } else if self.name() == "[J" {
            Object::new_data(_self, Box::new(vec![0_i64; count]))
        } else if self.name() == "[F" {
            Object::new_data(_self, Box::new(vec![0_f32; count]))
        } else if self.name() == "[D" {
            Object::new_data(_self, Box::new(vec![0_f64; count]))
        } else {
            Object::new_data(_self, Box::new(vec![None as OptionalRcRefCell<Object>; count]))
        }
    }

    pub fn component_class(&mut self) -> RcRefCell<Class> {
        let component_class_name = get_component_class_name(self.name().clone());
        let mut loader = self.loader();
        let loader = loader.as_mut().unwrap();
        return loader.borrow_mut().load_class(loader.clone(), component_class_name);
    }
}
