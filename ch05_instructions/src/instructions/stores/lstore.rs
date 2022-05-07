#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Store long into local variable
#[derive(Default, Debug)]
pub struct LSTORE {
    pub index: usize,
}

impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _lstore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_0;

impl Instruction for LSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _lstore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_1;

impl Instruction for LSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _lstore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_2;

impl Instruction for LSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _lstore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct LSTORE_3;

impl Instruction for LSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _lstore(frame, 3);
    }
}

fn _lstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack_mut().pop_long();
    frame.local_vars_mut().set_long(index, val);
}
