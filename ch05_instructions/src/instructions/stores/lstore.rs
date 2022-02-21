#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Store long into local variable
#[derive(Default, Debug)]
pub struct LSTORE {
    index: usize,
}

impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        _lstore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_0;

impl Instruction for LSTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        _lstore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_1;

impl Instruction for LSTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        _lstore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_2;

impl Instruction for LSTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        _lstore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_3;

impl Instruction for LSTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        _lstore(frame, 3);
    }
}

fn _lstore(frame: &mut Frame, index: usize) {
    let val = frame.get_operand_stack().pop_long();
    frame.get_local_vars().set_long(index, val);
}
