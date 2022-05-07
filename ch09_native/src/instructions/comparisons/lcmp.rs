#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Compare long
#[derive(Default, Debug)]
pub struct LCMP;

impl Instruction for LCMP {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v1 > v2 {
            stack.push_int(1);
        } else if v1 == v2 {
            stack.push_int(0);
        } else {
            stack.push_int(-1);
        }

        Ok(())
    }
}
