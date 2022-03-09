#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Load long from local variable
#[derive(Default, Debug)]
pub struct LLOAD {
    pub index: usize,
}

impl Instruction for LLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_0;

impl Instruction for LLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_1;

impl Instruction for LLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_2;

impl Instruction for LLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_3;

impl Instruction for LLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 3);
    }
}

fn _lload(frame: &mut Frame, index: usize) {
    let val = frame.get_local_vars().get_long(index);
    frame.get_operand_stack().push_long(val);
}
