#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Load double from local variable
#[derive(Default, Debug)]
pub struct DLOAD {
    pub index: usize,
}

impl Instruction for DLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _dload(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct DLOAD_0;

impl Instruction for DLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _dload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct DLOAD_1;

impl Instruction for DLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _dload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct DLOAD_2;

impl Instruction for DLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _dload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct DLOAD_3;

impl Instruction for DLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _dload(frame, 3);
    }
}

fn _dload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars_mut().get_double(index);
    frame.operand_stack_mut().push_double(val);
}
