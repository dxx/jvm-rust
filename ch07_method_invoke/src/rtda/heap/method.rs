use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::classfile::member_info::MemberInfo;
use super::access_flags::*;
use super::class::Class;
use super::method_descriptor::MethodDescriptorParser;
use std::rc::Rc;
use std::cell::RefCell;

pub fn new_methods(class: RcRefCell<Class>, cf_methods: &Vec<MemberInfo>) -> Vec<RcRefCell<Method>> {
    let mut methods = Vec::new();
    for m in cf_methods {
        let mut method = Method::default();
        method.class = Some(class.clone());
        method.copy_attributes(m);
        method.calc_arg_slot_count();
        methods.push(Rc::new(RefCell::new(method)));
    }
    methods
}

#[derive(Default)]
pub struct Method {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: OptionalRcRefCell<Class>,

    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,

    arg_slot_count: u64,
}

impl Method {
    pub fn copy_attributes(&mut self, cf_method: &MemberInfo) {
        self.access_flags = cf_method.access_flags();
        self.name = cf_method.name();
        self.descriptor = cf_method.descriptor();
        match cf_method.code_attribute() {
            Some(code_attr) => {
                self.max_stack = code_attr.max_stack();
                self.max_locals = code_attr.max_locals();
                self.code = code_attr.code();
            },
            None => {}
        }
    }

    pub fn calc_arg_slot_count(&mut self) {
        let parsed_descriptor = MethodDescriptorParser::parse(self.descriptor.clone());
        for param_type in parsed_descriptor.parameter_typs() {
            self.arg_slot_count += 1;
            if param_type == "J" || param_type == "D" {
                self.arg_slot_count += 1;
            }
        }
        if !self.is_static() {
            self.arg_slot_count += 1; // `this` reference
        }
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTECTED != 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.access_flags & ACC_SYNCHRONIZED != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.access_flags & ACC_BRIDGE != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.access_flags & ACC_VARARGS != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags & ACC_NATIVE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_strict(&self) -> bool {
        self.access_flags & ACC_STRICT != 0
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn get_class(&self) -> RcRefCell<Class> {
        self.class.clone().unwrap()
    }

    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn arg_slot_count(&self) -> u64 {
        self.arg_slot_count
    }

    /// jvms 5.4.4
    pub fn is_accessible_to(&self, class: &RcRefCell<Class>) -> bool {
        if self.is_public() {
            return true;
        }
        let c = self.class.as_ref().unwrap();
        if self.is_protected() {
            return class.eq(c) || class.borrow().is_sub_class_of(c) ||
            c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        if !self.is_private() {
            return c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        return class.eq(c);
    }
}
