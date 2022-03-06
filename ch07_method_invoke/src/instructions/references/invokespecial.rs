#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_methodref::MethodRef;
use crate::rtda::method_lookup::lookup_method_in_class;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::invoke_method;

/// Invoke instance method;
/// Special handling for superclass, private, and instance initialization method invocations
#[derive(Default, Debug)]
pub struct INVOKE_SPECIAL {
    index: u64,
}

impl Instruction for INVOKE_SPECIAL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.get_method().borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();    
        let resolved_class = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<MethodRef>().unwrap().resolved_class(current_class.clone());
        let resolved_method = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<MethodRef>().unwrap().resolved_method(resolved_class.clone());

        if resolved_method.borrow().name() == "<init>" && resolved_method.borrow().get_class().ne(&resolved_class) {
            panic!("java.lang.NoSuchMethodError");
        }

        if resolved_method.borrow().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let _ref = frame.get_operand_stack().get_ref_from_top(
            (resolved_method.borrow().arg_slot_count() - 1) as usize);
        if _ref.is_none() {
            panic!("java.lang.NullPointerException");
        }

        if resolved_method.borrow().is_protected() &&
            resolved_method.borrow().get_class().borrow().is_sub_class_of(&current_class) &&
            resolved_method.borrow().get_class().borrow().get_package_name() != current_class.borrow().get_package_name() &&
            _ref.as_ref().unwrap().borrow().class().ne(&current_class) &&
            !_ref.as_ref().unwrap().borrow().class().borrow().is_sub_class_of(&current_class) {
            panic!("java.lang.IllegalAccessError");
        }

        let mut method_to_be_invoked = Some(resolved_method.clone());
        if current_class.borrow().is_super() &&
            resolved_class.borrow().is_sub_class_of(&current_class) &&
            resolved_method.borrow().name() != "<init>" {
            method_to_be_invoked = lookup_method_in_class(current_class.borrow().super_class().as_ref().unwrap(),
                resolved_method.borrow().name(),
                resolved_method.borrow().descriptor());
        }

        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().borrow().is_abstract() {
            panic!("java.lang.AbstractMethodError");
        }

        invoke_method(frame, method_to_be_invoked.as_ref().unwrap());
    }
}
