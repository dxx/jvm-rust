#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

#[derive(Default, Debug)]
pub struct WIDE;

impl Instruction for WIDE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {

    }

    fn execute(&self, frame: &mut Frame) {

    }
}