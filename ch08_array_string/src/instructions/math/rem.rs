#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Remainder double
#[derive(Default, Debug)]
pub struct DREM;

impl Instruction for DREM {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let result = v1 % v2;
        stack.push_double(result);

        Ok(())
    }
}

/// Remainder flaot
#[derive(Default, Debug)]
pub struct FREM;

impl Instruction for FREM {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1 % v2;
        stack.push_float(result);

        Ok(())
    }
}

/// Remainder int
#[derive(Default, Debug)]
pub struct IREM;

impl Instruction for IREM {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        if v2 == 0 {
            return Err("java.lang.ArithmeticException: / by zero".into());
        }
        let result = v1 % v2;
        stack.push_int(result);

        Ok(())
    }
}

/// Remainder long
#[derive(Default, Debug)]
pub struct LREM;

impl Instruction for LREM {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.get_operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v2 == 0 {
            return Err("java.lang.ArithmeticException: / by zero".into());
        }
        let result = v1 % v2;
        stack.push_long(result);

        Ok(())
    }
}
