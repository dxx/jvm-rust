#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Duplicate the top operand stack value
#[derive(Default, Debug)]
pub struct DUP;

impl Instruction for DUP {
    /// bottom -> top
    /// [...][c][b][a]
    ///             \ _
    ///                |
    ///                V
    /// [...][c][b][a][a]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot = stack.pop_slot();
        let slot2 = slot.clone();
        stack.push_slot(slot);
        stack.push_slot(slot2);

        Ok(())
    }
}

/// Duplicate the top operand stack value and insert two values down
#[derive(Default, Debug)]
pub struct DUP_X1;

impl Instruction for DUP_X1 {
    /// bottom -> top
    /// [...][c][b][a]
    ///           _/
    ///          |
    ///          V
    /// [...][c][a][b][a]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        stack.push_slot(slot1.clone());
        stack.push_slot(slot2);
        stack.push_slot(slot1);

        Ok(())
    }
}

/// Duplicate the top operand stack value and insert two or three values down
#[derive(Default, Debug)]
pub struct DUP_X2;

impl Instruction for DUP_X2 {
    /// bottom -> top
    /// [...][c][b][a]
    ///        ____/
    ///       |
    ///       V
    /// [...][a][c][b][a]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = stack.pop_slot();
        stack.push_slot(slot1.clone());
        stack.push_slot(slot3);
        stack.push_slot(slot2);
        stack.push_slot(slot1);

        Ok(())
    }
}

/// Duplicate the top one or two operand stack values
#[derive(Default, Debug)]
pub struct DUP2;

impl Instruction for DUP2 {
    /// bottom -> top
    /// [...][c][b][a]_____
    ///           \_____  |
    ///                |  |
    ///                V  V
    /// [...][c][b][a][b][a]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        stack.push_slot(slot2.clone());
        stack.push_slot(slot1.clone());
        stack.push_slot(slot2);
        stack.push_slot(slot1);

        Ok(())
    }
}

/// Duplicate the top one or two operand stack values and insert two or three values down
#[derive(Default, Debug)]
pub struct DUP2_X1;

impl Instruction for DUP2_X1 {
    /// bottom -> top
    /// [...][c][b][a]
    ///        _/ _/
    ///       |  |
    ///       V  V
    /// [...][b][a][c][b][a]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = stack.pop_slot();
        stack.push_slot(slot2.clone());
        stack.push_slot(slot1.clone());
        stack.push_slot(slot3);
        stack.push_slot(slot2);
        stack.push_slot(slot1);

        Ok(())
    }
}

/// Duplicate the top one or two operand stack values and insert two, three, or four values down
#[derive(Default, Debug)]
pub struct DUP2_X2;

impl Instruction for DUP2_X2 {
    /// bottom -> top
    /// [...][d][c][b][a]
    ///        ____/ __/
    ///       |   __/
    ///       V  V
    /// [...][b][a][d][c][b][a]
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = stack.pop_slot();
        let slot4 = stack.pop_slot();
        stack.push_slot(slot2.clone());
        stack.push_slot(slot1.clone());
        stack.push_slot(slot4);
        stack.push_slot(slot3);
        stack.push_slot(slot2);
        stack.push_slot(slot1);

        Ok(())
    }
}
