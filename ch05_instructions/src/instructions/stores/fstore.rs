#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Store float into local variable
#[derive(Default, Debug)]
pub struct FSTORE {
    pub index: usize,
}

impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _fstore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_0;

impl Instruction for FSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _fstore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_1;

impl Instruction for FSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _fstore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_2;

impl Instruction for FSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _fstore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_3;

impl Instruction for FSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _fstore(frame, 3);
    }
}

fn _fstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack_mut().pop_float();
    frame.local_vars_mut().set_float(index, val);
}
