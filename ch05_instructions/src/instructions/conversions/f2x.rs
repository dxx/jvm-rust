#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Convert float to double
#[derive(Default, Debug)]
pub struct F2D;

impl Instruction for F2D {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let f = stack.pop_float();
        let d = f as f64;
        stack.push_double(d);
    }
}

/// Convert float to int
#[derive(Default, Debug)]
pub struct F2I;

impl Instruction for F2I {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let f = stack.pop_float();
        let i = f as i32;
        stack.push_int(i);
    }
}

/// Convert float to long
#[derive(Default, Debug)]
pub struct F2L;

impl Instruction for F2L {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let f = stack.pop_float();
        let l = f as i64;
        stack.push_long(l);
    }
}
