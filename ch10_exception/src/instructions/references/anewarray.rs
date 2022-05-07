#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_classref::ClassRef;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;
use std::rc::Rc;
use std::cell::RefCell;

/// Create new array of reference
#[derive(Default, Debug)]
pub struct ANEW_ARRAY {
    index: u64, 
}

impl Instruction for ANEW_ARRAY {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let method = frame.method();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let component_class = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<ClassRef>().unwrap().resolved_class(current_class);

        let stack = frame.operand_stack_mut();
        let count = stack.pop_int();
        if count < 0 {
            return Err("java.lang.NegativeArraySizeException".into());
        }

        let arr_class = component_class.borrow_mut().array_class();
        let arr = arr_class.borrow_mut().new_array(arr_class.clone(), count as usize);
        stack.push_ref(Some(Rc::new(RefCell::new(arr))));

        Ok(())
    }
}