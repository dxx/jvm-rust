#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

// Boolean OR int
#[derive(Default, Debug)]
pub struct IOR;

impl Instruction for IOR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let result = v1 | v2;
        stack.push_int(result);

        Ok(())
    }
}

// Boolean OR long
#[derive(Default, Debug)]
pub struct LOR;

impl Instruction for LOR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let result = v1 | v2;
        stack.push_long(result);

        Ok(())
    }
}
