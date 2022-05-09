#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Convert double to float
#[derive(Default, Debug)]
pub struct D2F;

impl Instruction for D2F {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let d = stack.pop_double();
        let f = d as f32;
        stack.push_float(f);

        Ok(())
    }
}

/// Convert double to int
#[derive(Default, Debug)]
pub struct D2I;

impl Instruction for D2I {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let d = stack.pop_double();
        let i = d as i32;
        stack.push_int(i);

        Ok(())
    }
}

/// Convert double to long
#[derive(Default, Debug)]
pub struct D2L;

impl Instruction for D2L {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let d = stack.pop_double();
        let l = d as i64;
        stack.push_long(l);

        Ok(())
    }
}
