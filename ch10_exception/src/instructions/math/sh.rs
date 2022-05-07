#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Shift left int
#[derive(Default, Debug)]
pub struct ISHL;

impl Instruction for ISHL {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = v2 as u32 & 0x1f;
        let result = v1 << s;
        stack.push_int(result);

        Ok(())
    }
}

/// Arithmetic shift right int
#[derive(Default, Debug)]
pub struct ISHR;

impl Instruction for ISHR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = v2 as u32 & 0x1f;
        let result = v1 >> s;
        stack.push_int(result);

        Ok(())
    }
}

/// Logical shift right int
#[derive(Default, Debug)]
pub struct IUSHR;

impl Instruction for IUSHR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = v2 as u32 & 0x1f;
        let result = (v1 as u32 >> s) as i32;
        stack.push_int(result);

        Ok(())
    }
}

/// Shift left long
#[derive(Default, Debug)]
pub struct LSHL;

impl Instruction for LSHL {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = v2 as u32 & 0x3f;
        let result = v1 << s;
        stack.push_long(result);

        Ok(())
    }
}

/// Arithmetic shift right long
#[derive(Default, Debug)]
pub struct LSHR;

impl Instruction for LSHR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = v2 as u32 & 0x3f;
        let result = v1 >> s;
        stack.push_long(result);

        Ok(())
    }
}

/// Logical shift right long
#[derive(Default, Debug)]
pub struct LUSHR;

impl Instruction for LUSHR {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = v2 as u32 & 0x3f;
        let result = (v1 as u64 >> s) as i64;
        stack.push_long(result);

        Ok(())
    }
}
