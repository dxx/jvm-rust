#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

// Boolean XOR int
#[derive(Default, Debug)]
pub struct IXOR;

impl Instruction for IXOR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v1 = stack.pop_int();
        let v2 = stack.pop_int();
        let result = v1 ^ v2;
        stack.push_int(result);

        Ok(())
    }
}

// Boolean XOR long
#[derive(Default, Debug)]
pub struct LXOR;

impl Instruction for LXOR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v1 = stack.pop_long();
        let v2 = stack.pop_long();
        let result = v1 ^ v2;
        stack.push_long(result);

        Ok(())
    }
}
