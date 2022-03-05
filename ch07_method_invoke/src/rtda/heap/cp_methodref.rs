use crate::classfile::constant_pool::cp_member_ref::ConstantMethodRefInfo;
use crate::classfile::constant_pool::CONSTANT_METHOD_REF;
use super::class::Class;
use super::method::Method;
use super::constant_pool::Constant;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MethodRef {
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
    name: String,
    descriptor: String,
    method: Option<Rc<RefCell<Method>>>,
}

impl MethodRef {
    pub fn new(ref_info: &ConstantMethodRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        MethodRef {
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            method: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn resolved_method(&mut self) -> Rc<RefCell<Method>> {
        if self.method.is_none() {
            self.resolve_method_ref();
        }
        self.method.clone().unwrap()
    }

    /// jvms8 5.4.3.3
    fn resolve_method_ref(&self) {

    }
}

impl Constant for MethodRef {
    fn tag(&self) -> u8 {
        CONSTANT_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
