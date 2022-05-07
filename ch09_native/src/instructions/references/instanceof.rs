#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_classref::ClassRef;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Determine if object is of given type
#[derive(Default, Debug)]
pub struct INSTANCE_OF {
    index: u64,
}

impl Instruction for INSTANCE_OF {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }
    
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let method = frame.method();
        let stack = frame.operand_stack_mut();
        let _ref = stack.pop_ref();
        if _ref.is_none() {
            stack.push_int(0);
            return Ok(());
        }

        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let class = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<ClassRef>().unwrap().resolved_class(current_class);

        if _ref.unwrap().borrow().is_instance_of(&class) {
            stack.push_int(1);
        } else {
            stack.push_int(0);
        }

        Ok(())
    }
}
