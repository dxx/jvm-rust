#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Multiply double
#[derive(Default, Debug)]
pub struct DMUL;

impl Instruction for DMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let result = v1 * v2;
        stack.push_double(result);
    }
}

/// Multiply float
#[derive(Default, Debug)]
pub struct FMUL;

impl Instruction for FMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1 * v2;
        stack.push_float(result);
    }
}

/// Multiply int
#[derive(Default, Debug)]
pub struct IMUL;

impl Instruction for IMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let result = v1 * v2;
        stack.push_int(result);
    }
}

/// Multiply long
#[derive(Default, Debug)]
pub struct LMUL;

impl Instruction for LMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let result = v1 * v2;
        stack.push_long(result);
    }
}
