use crate::classfile::constant_pool::cp_member_ref::ConstantFieldRefInfo;
use crate::classfile::constant_pool::CONSTANT_FIELD_REF;
use super::class::Class;
use super::constant_pool::{Constant, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

pub struct FieldRef {
    cp: Rc<RefCell<ConstantPool>>,
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
    name: String,
    descriptor: String,
}

impl FieldRef {
    pub fn new(cp: Rc<RefCell<ConstantPool>>, ref_info: &ConstantFieldRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        FieldRef {
            cp,
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
        }
    }
}

impl Constant for FieldRef {
    fn tag(&self) -> u8 {
        CONSTANT_FIELD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
