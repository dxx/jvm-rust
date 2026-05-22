#![allow(non_camel_case_types)]

use super::super::instruction::Instruction;
use crate::rtda::Frame;

/// Negate double
#[derive(Default, Debug)]
pub struct DNEG;

impl Instruction for DNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let val = stack.pop_double();
        stack.push_double(-val);
    }
}

/// Negate float
#[derive(Default, Debug)]
pub struct FNEG;

impl Instruction for FNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let val = stack.pop_float();
        stack.push_float(-val);
    }
}

/// Negate int
#[derive(Default, Debug)]
pub struct INEG;

impl Instruction for INEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let val = stack.pop_int();
        stack.push_int(-val);
    }
}

/// Negate long
#[derive(Default, Debug)]
pub struct LNEG;

impl Instruction for LNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let val = stack.pop_long();
        stack.push_long(-val);
    }
}
