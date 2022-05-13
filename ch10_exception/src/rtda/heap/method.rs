use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::classfile::member_info::MemberInfo;
use crate::classfile::attribute_info::attr_line_number_table::LineNumberTableAttribute;
use super::access_flags::*;
use super::class::Class;
use super::exception_table::ExceptionTable;
use super::method_descriptor::MethodDescriptorParser;
use std::rc::Rc;
use std::cell::RefCell;

pub fn new_methods(class: RcRefCell<Class>, cf_methods: &Vec<MemberInfo>) -> Vec<RcRefCell<Method>> {
    let mut methods = Vec::new();
    for m in cf_methods {
        let method = Method::new(class.clone(), m);
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

    exception_table: Option<ExceptionTable>,
    line_number_table: Option<LineNumberTableAttribute>,
}

impl Method {
    pub fn new(class: RcRefCell<Class>, cf_method: &MemberInfo) -> Self {
        let mut method = Method::default();
        method.class = Some(class);
        method.copy_attributes(cf_method);
        let parsed_descriptor = MethodDescriptorParser::parse(method.descriptor.clone());
        method.calc_arg_slot_count(parsed_descriptor.parameter_typs());
        if method.is_native() {
            method.inject_code_attribute(parsed_descriptor.return_type());
        }
        method
    }

    pub fn copy_attributes(&mut self, cf_method: &MemberInfo) {
        self.access_flags = cf_method.access_flags();
        self.name = cf_method.name();
        self.descriptor = cf_method.descriptor();
        match cf_method.code_attribute() {
            Some(code_attr) => {
                self.max_stack = code_attr.max_stack();
                self.max_locals = code_attr.max_locals();
                self.code = code_attr.code();
                self.exception_table = Some(ExceptionTable::new(
                    code_attr.exception_table(),
                    &self.class.as_ref().unwrap().borrow().constant_pool()));
                self.line_number_table = code_attr.line_number_table_attribute();
            },
            None => {}
        }
    }

    pub fn calc_arg_slot_count(&mut self, parameter_typs: Vec<String>) {
        for param_type in parameter_typs {
            self.arg_slot_count += 1;
            if param_type == "J" || param_type == "D" {
                self.arg_slot_count += 1;
            }
        }
        if !self.is_static() {
            self.arg_slot_count += 1; // `this` reference
        }
    }

    pub fn inject_code_attribute(&mut self, return_type: String) {
        self.max_stack = 4; // Todo
        self.max_locals = self.arg_slot_count as u16;
        match return_type.as_bytes()[0] {
            b'L' | b'[' => {
                self.code = vec![0xfe, 0xb0]; // areturn
            },
            b'V' => {
                self.code = vec![0xfe, 0xb1]; // return
            },
            b'D' => {
                self.code = vec![0xfe, 0xaf]; // dreturn
            },
            b'F' => {
                self.code = vec![0xfe, 0xae]; // freturn
            },
            b'J' => {
                self.code = vec![0xfe, 0xad]; // lreturn
            },
            _ => {
                self.code = vec![0xfe, 0xac]; // ireturn
            }
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

    pub fn find_exception_handler(&mut self, ex_class: &RcRefCell<Class>, pc: i64) -> i64 {
        let handler = self.exception_table.as_mut().unwrap().find_exception_handler(ex_class, pc);
        if handler.is_some() {
            return handler.unwrap().handler_pc();
        }

        -1
    }

    pub fn get_line_number(&self, pc: i64) -> i64 {
        if self.is_native() {
            return -2;
        }
        if self.line_number_table.is_none() {
            return -1;
        }
        self.line_number_table.as_ref().unwrap().get_line_number(pc)
    }
}
