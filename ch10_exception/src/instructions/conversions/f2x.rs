#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Convert float to double
#[derive(Default, Debug)]
pub struct F2D;

impl Instruction for F2D {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let f = stack.pop_float();
        let d = f as f64;
        stack.push_double(d);

        Ok(())
    }
}

/// Convert float to int
#[derive(Default, Debug)]
pub struct F2I;

impl Instruction for F2I {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let f = stack.pop_float();
        let i = f as i32;
        stack.push_int(i);

        Ok(())
    }
}

/// Convert float to long
#[derive(Default, Debug)]
pub struct F2L;

impl Instruction for F2L {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let f = stack.pop_float();
        let l = f as i64;
        stack.push_long(l);

        Ok(())
    }
}
