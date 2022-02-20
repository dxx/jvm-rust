#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Pop the top operand stack value
#[derive(Default, Debug)]
pub struct POP;

impl Instruction for POP {
    /// bottom -> top
    /// [...][c][b][a]
    ///             |
    ///             V
    /// [...][c][b]
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().pop_slot();
    }
}

/// Pop the top one or two operand stack values
#[derive(Default, Debug)]
pub struct POP2;

impl Instruction for POP2 {
    /// bottom -> top
    /// [...][c][b][a]
    ///          |  |
    ///          V  V
    /// [...][c]
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        stack.pop_slot();
        stack.pop_slot();
    }
}
