#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

#[derive(Default, Debug)]
pub struct ACONST_NULL;

impl Instruction for ACONST_NULL {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        // Push None
        frame.get_operand_stack().push_ref(None);

        Ok(())
    }
}

/// Push double constant
#[derive(Default, Debug)]
pub struct DCONST_0;

impl Instruction for DCONST_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_double(0.0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DCONST_1;

impl Instruction for DCONST_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_double(1.0);

        Ok(())
    }
}

/// Push float constant
#[derive(Default, Debug)]
pub struct FCONST_0;

impl Instruction for FCONST_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_float(0.0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FCONST_1;

impl Instruction for FCONST_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_float(1.0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FCONST_2;

impl Instruction for FCONST_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_float(2.0);

        Ok(())
    }
}

/// Push int constant
#[derive(Default, Debug)]
pub struct ICONST_M1;

impl Instruction for ICONST_M1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(-1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ICONST_0;

impl Instruction for ICONST_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ICONST_1;

impl Instruction for ICONST_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ICONST_2;

impl Instruction for ICONST_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ICONST_3;

impl Instruction for ICONST_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(3);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ICONST_4;

impl Instruction for ICONST_4 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(4);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ICONST_5;

impl Instruction for ICONST_5 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_int(5);

        Ok(())
    }
}

/// Push long constant
#[derive(Default, Debug)]
pub struct LCONST_0;

impl Instruction for LCONST_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_long(0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct LCONST_1;

impl Instruction for LCONST_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_operand_stack().push_long(1);

        Ok(())
    }
}
