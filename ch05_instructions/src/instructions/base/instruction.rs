use std::fmt::Debug;

use crate::rtda::Frame;
use super::bytecode_reader::BytecodeReader;

pub trait Instruction: Debug {
    // fn fetch_operands(&mut self, reader: &mut BytecodeReader);
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // Nothing to do
    }
    
    fn execute(&self, frame: &mut Frame);
}

#[derive(Default, Debug)]
pub struct BranchInstruction {
    pub offset: i32,
}

impl Instruction for BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        // Nothing to do
    }
}

#[derive(Default, Debug)]
pub struct Index8Instruction {
    pub index: u32,
}

impl Instruction for Index8Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as u32;
    }

    fn execute(&self, frame: &mut Frame) {
        // Nothing to do
    }
}

#[derive(Default, Debug)]
pub struct Index16Instruction {
    pub index: u32,
}

impl Instruction for Index16Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u32;
    }

    fn execute(&self, frame: &mut Frame) {
        // Nothing to do
    }
}
