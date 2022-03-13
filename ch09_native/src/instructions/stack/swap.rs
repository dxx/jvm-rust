#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Swap the top two operand stack values
#[derive(Default, Debug)]
pub struct SWAP;

impl Instruction for SWAP {
    /// bottom -> top
    /// [...][c][b][a]
    ///           \/
    ///           /\
    ///          V  V
    /// [...][c][a][b]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack .pop_slot();
        stack.push_slot(slot1);
        stack.push_slot(slot2);

        Ok(())
    }
}
