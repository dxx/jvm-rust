use crate::classfile::constant_pool::cp_member_ref::ConstantInterfaceMethodRefInfo;
use crate::classfile::constant_pool::CONSTANT_INTERFACE_METHOD_REF;
use super::class::Class;
use super::method::Method;
use super::constant_pool::{Constant, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

pub struct InterfaceMethodRef {
    cp: Rc<RefCell<ConstantPool>>,
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
    name: String,
    descriptor: String,
    method: Option<Rc<RefCell<Method>>>,
}

impl InterfaceMethodRef {
    pub fn new(cp: Rc<RefCell<ConstantPool>>, ref_info: &ConstantInterfaceMethodRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        InterfaceMethodRef {
            cp,
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            method: None,
        }
    }

    pub fn resolved_interface_method(&mut self) -> Rc<RefCell<Method>> {
        if self.method.is_none() {
            self.resolve_interface_method_ref();
        }
        self.method.clone().unwrap()
    }

    /// jvms8 5.4.3.4
    fn resolve_interface_method_ref(&self) {

    }
}

impl Constant for InterfaceMethodRef {
    fn tag(&self) -> u8 {
        CONSTANT_INTERFACE_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
