#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_methodref::MethodRef;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Invoke instance method; dispatch based on class
#[derive(Default, Debug)]
pub struct INVOKE_VIRTUAL {
    index: u64,
}

impl Instruction for INVOKE_VIRTUAL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    /// Hack!
    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let cp = r_cp.borrow();
        let method_ref = cp.get_constant(self.index as usize)
            .as_any().downcast_ref::<MethodRef>().unwrap();

        if method_ref.name() == "println" {
            let stack = frame.operand_stack_mut();
            let descriptor = method_ref.descriptor();
            if descriptor == "(Z)V" {
                println!("{}", stack.pop_int() != 0);
            } else if descriptor == "(C)V" {
                println!("{}", stack.pop_int());
            } else if descriptor == "(I)V" || descriptor == "(B)V" || descriptor == "(S)V" {
                println!("{}", stack.pop_int());
            } else if descriptor == "(F)V" {
                println!("{}", stack.pop_float());
            } else if descriptor == "(J)V" {
                println!("{}", stack.pop_long());
            } else if descriptor == "(D)V" {
                println!("{}", stack.pop_double());
            } else {
                panic!("println: {}", descriptor);
            }
            stack.pop_ref();
        }
    }
}
