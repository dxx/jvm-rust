#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

// Boolean OR int
#[derive(Default, Debug)]
pub struct IOR;

impl Instruction for IOR {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let result = v1 | v2;
        stack.push_int(result);
    }
}

// Boolean OR long
#[derive(Default, Debug)]
pub struct LOR;

impl Instruction for LOR {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let result = v1 | v2;
        stack.push_long(result);
    }
}
