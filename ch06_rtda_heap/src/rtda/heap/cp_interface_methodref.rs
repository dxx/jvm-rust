use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::classfile::constant_pool::cp_member_ref::ConstantInterfaceMethodRefInfo;
use crate::classfile::constant_pool::CONSTANT_INTERFACE_METHOD_REF;
use super::class::Class;
use super::method::Method;
use super::constant_pool::Constant;

pub struct InterfaceMethodRef {
    class_name: String,
    class: OptionalRcRefCell<Class>,
    name: String,
    descriptor: String,
    method: OptionalRcRefCell<Method>,
}

impl InterfaceMethodRef {
    pub fn new(ref_info: &ConstantInterfaceMethodRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        InterfaceMethodRef {
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            method: None,
        }
    }

    pub fn resolved_interface_method(&mut self) -> RcRefCell<Method> {
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
