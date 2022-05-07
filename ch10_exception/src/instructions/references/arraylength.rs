#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Get length of array
#[derive(Default, Debug)]
pub struct ARRAY_LENGTH;

impl Instruction for ARRAY_LENGTH {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let _ref = stack.pop_ref();
        if _ref.is_none() {
            panic!("java.lang.NullPointerException");
        }

        let arr_len = _ref.unwrap().borrow().array_length();
        stack.push_int(arr_len as i32);

        Ok(())
    }
}
