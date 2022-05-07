#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_interface_methodref::InterfaceMethodRef;
use crate::rtda::method_lookup::lookup_method_in_class;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::invoke_method;

/// Invoke interface method
#[derive(Default, Debug)]
pub struct INVOKE_INTERFACE {
    index: u64,
    // count: u8,
    // zero: u8,
}

impl Instruction for INVOKE_INTERFACE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
        reader.read_u8(); // count
        reader.read_u8(); // must be 0
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let resolved_class = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<InterfaceMethodRef>().unwrap().resolved_class(current_class.clone());
        let resolved_method = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<InterfaceMethodRef>().unwrap().resolved_interface_method(resolved_class.clone());

        if resolved_method.borrow().is_static() || resolved_method.borrow().is_private() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let _ref = frame.operand_stack_mut().get_ref_from_top(
            (resolved_method.borrow().arg_slot_count() - 1) as usize);
        if _ref.is_none() {
            panic!("java.lang.NullPointerException");
        }
        if !_ref.as_ref().unwrap().borrow().class().borrow().is_implements(&resolved_class) {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let method_to_be_invoked = lookup_method_in_class(
            _ref.as_ref().unwrap().borrow().class(),
            resolved_method.borrow().name(),
            resolved_method.borrow().descriptor());

        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().borrow().is_abstract() {
            panic!("java.lang.AbstractMethodError");
        }
        if !method_to_be_invoked.as_ref().unwrap().borrow().is_public() {
            panic!("java.lang.IllegalAccessError");
        }

        invoke_method(frame, method_to_be_invoked.as_ref().unwrap());
    }
}

