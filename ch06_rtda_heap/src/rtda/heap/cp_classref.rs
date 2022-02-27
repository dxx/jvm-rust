use crate::classfile::constant_pool::cp_class::ConstantClassInfo;
use crate::classfile::constant_pool::CONSTANT_CLASS;
use super::class::Class;
use super::constant_pool::{Constant, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ClassRef {
    cp: Rc<RefCell<ConstantPool>>,
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
}

impl ClassRef {
    pub fn new(cp: Rc<RefCell<ConstantPool>>, class_info: &ConstantClassInfo) -> Self {
        ClassRef {
            cp,
            class_name: class_info.name(),
            class: None,
        }
    }
}

impl Constant for ClassRef {
    fn tag(&self) -> u8 {
        CONSTANT_CLASS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
