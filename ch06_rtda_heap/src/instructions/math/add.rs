#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Add double
#[derive(Default, Debug)]
pub struct DADD;

impl Instruction for DADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let result = v1 + v2;
        stack.push_double(result);
    }
}

/// Add float
#[derive(Default, Debug)]
pub struct FADD;

impl Instruction for FADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1 + v2;
        stack.push_float(result);
    }
}

/// Add int
#[derive(Default, Debug)]
pub struct IADD;

impl Instruction for IADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let result = v1 + v2;
        stack.push_int(result);
    }
}

/// Add long
#[derive(Default, Debug)]
pub struct LADD;

impl Instruction for LADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let result = v1 + v2;
        stack.push_long(result);
    }
}