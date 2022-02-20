#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

#[derive(Default, Debug)]
pub struct ACONST_NULL;

impl Instruction for ACONST_NULL {
    fn execute(&self, frame: &mut Frame) {
        // Push None
        frame.get_operand_stack().push_ref(None);
    }
}

/// Push double constant
#[derive(Default, Debug)]
pub struct DCONST_0;

impl Instruction for DCONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_double(0.0);
    }
}

#[derive(Default, Debug)]
pub struct DCONST_1;

impl Instruction for DCONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_double(1.0);
    }
}

/// Push float constant
#[derive(Default, Debug)]
pub struct FCONST_0;

impl Instruction for FCONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_float(0.0);
    }
}

#[derive(Default, Debug)]
pub struct FCONST_1;

impl Instruction for FCONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_float(1.0);
    }
}

#[derive(Default, Debug)]
pub struct FCONST_2;

impl Instruction for FCONST_2 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_float(2.0);
    }
}

/// Push int constant
#[derive(Default, Debug)]
pub struct ICONST_M1;

impl Instruction for ICONST_M1 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(-1);
    }
}

#[derive(Default, Debug)]
pub struct ICONST_0;

impl Instruction for ICONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(0);
    }
}

#[derive(Default, Debug)]
pub struct ICONST_1;

impl Instruction for ICONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(1);
    }
}

#[derive(Default, Debug)]
pub struct ICONST_2;

impl Instruction for ICONST_2 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(2);
    }
}

#[derive(Default, Debug)]
pub struct ICONST_3;

impl Instruction for ICONST_3 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(3);
    }
}

#[derive(Default, Debug)]
pub struct ICONST_4;

impl Instruction for ICONST_4 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(4);
    }
}

#[derive(Default, Debug)]
pub struct ICONST_5;

impl Instruction for ICONST_5 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_int(5);
    }
}

/// Push long constant
#[derive(Default, Debug)]
pub struct LCONST_0;

impl Instruction for LCONST_0 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_long(0);
    }
}

#[derive(Default, Debug)]
pub struct LCONST_1;

impl Instruction for LCONST_1 {
    fn execute(&self, frame: &mut Frame) {
        frame.get_operand_stack().push_long(1);
    }
}
