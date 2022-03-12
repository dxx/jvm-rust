#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Negate double
#[derive(Default, Debug)]
pub struct DNEG;

impl Instruction for DNEG {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let val = stack.pop_double();
        stack.push_double(-val);

        Ok(())
    }
}

/// Negate float
#[derive(Default, Debug)]
pub struct FNEG;

impl Instruction for FNEG {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let val = stack.pop_float();
        stack.push_float(-val);

        Ok(())
    }
}

/// Negate int
#[derive(Default, Debug)]
pub struct INEG;

impl Instruction for INEG {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let val = stack.pop_int();
        stack.push_int(-val);

        Ok(())
    }
}

/// Negate long
#[derive(Default, Debug)]
pub struct LNEG;

impl Instruction for LNEG {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let val = stack.pop_long();
        stack.push_long(-val);

        Ok(())
    }
}
