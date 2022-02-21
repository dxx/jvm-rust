#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Store reference into local variable
#[derive(Default, Debug)]
pub struct ASTORE {
    index: usize,
}

impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        _astore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_0;

impl Instruction for ASTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        _astore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_1;

impl Instruction for ASTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        _astore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_2;

impl Instruction for ASTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        _astore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_3;

impl Instruction for ASTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        _astore(frame, 3);
    }
}

fn _astore(frame: &mut Frame, index: usize) {
    let val = frame.get_operand_stack().pop_ref();
    frame.get_local_vars().set_ref(index, val);
}
