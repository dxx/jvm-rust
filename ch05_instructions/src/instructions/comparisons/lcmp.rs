#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Compare long
#[derive(Default, Debug)]
pub struct LCMP;

impl Instruction for LCMP {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v1 > v2 {
            stack.push_int(1);
        } else if v1 == v2 {
            stack.push_int(0);
        } else {
            stack.push_int(-1);
        }
    }
}
