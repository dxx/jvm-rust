#![allow(non_camel_case_types)]

use super::super::branch;
use super::super::bytecode_reader::BytecodeReader;
use super::super::instruction::Instruction;
use crate::rtda::Frame;

/// Branch always
#[derive(Default, Debug)]
pub struct GOTO {
    pub offset: i64,
}

impl Instruction for GOTO {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        branch(frame, self.offset);
    }
}
