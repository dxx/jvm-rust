use crate::classfile::constant_pool::cp_member_ref::ConstantMethodRefInfo;
use crate::classfile::constant_pool::CONSTANT_METHOD_REF;
use super::class::Class;
use super::constant_pool::{Constant, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MethodRef {
    cp: Rc<RefCell<ConstantPool>>,
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
    name: String,
    descriptor: String,
}

impl MethodRef {
    pub fn new(cp: Rc<RefCell<ConstantPool>>, ref_info: &ConstantMethodRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        MethodRef {
            cp,
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
        }
    }
}

impl Constant for MethodRef {
    fn tag(&self) -> u8 {
        CONSTANT_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
