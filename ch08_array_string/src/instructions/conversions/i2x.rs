#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Convert int to byte
#[derive(Default, Debug)]
pub struct I2B;

impl Instruction for I2B {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let i = stack.pop_int();
        let b = i as i8 as i32;
        stack.push_int(b);

        Ok(())
    }
}

/// Convert int to char
#[derive(Default, Debug)]
pub struct I2C;

impl Instruction for I2C {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let i = stack.pop_int();
        let c = i as u16 as i32;
        stack.push_int(c);

        Ok(())
    }
}

/// Convert int to short
#[derive(Default, Debug)]
pub struct I2S;

impl Instruction for I2S {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let i = stack.pop_int();
        let s = i as i16 as i32;
        stack.push_int(s);

        Ok(())
    }
}

/// Convert int to long
#[derive(Default, Debug)]
pub struct I2L;

impl Instruction for I2L {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let i = stack.pop_int();
        let l = i as i64;
        stack.push_long(l);

        Ok(())
    }
}

/// Convert int to float
#[derive(Default, Debug)]
pub struct I2F;

impl Instruction for I2F {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let i = stack.pop_int();
        let f = i as f32;
        stack.push_float(f);

        Ok(())
    }
}

/// Convert int to double
#[derive(Default, Debug)]
pub struct I2D;

impl Instruction for I2D {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let i = stack.pop_int();
        let d = i as f64;
        stack.push_double(d);

        Ok(())
    }
}
