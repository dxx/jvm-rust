#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Store int into local variable
#[derive(Default, Debug)]
pub struct ISTORE {
    index: usize,
}

impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        _istore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_0;

impl Instruction for ISTORE_0 {
    fn execute(&self, frame: &mut Frame) {
        _istore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_1;

impl Instruction for ISTORE_1 {
    fn execute(&self, frame: &mut Frame) {
        _istore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_2;

impl Instruction for ISTORE_2 {
    fn execute(&self, frame: &mut Frame) {
        _istore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_3;

impl Instruction for ISTORE_3 {
    fn execute(&self, frame: &mut Frame) {
        _istore(frame, 3);
    }
}

fn _istore(frame: &mut Frame, index: usize) {
    let val = frame.get_operand_stack().pop_int();
    frame.get_local_vars().set_int(index, val);
}
