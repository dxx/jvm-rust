use std::fmt::Debug;

use crate::rtda::Frame;
use super::bytecode_reader::BytecodeReader;

pub trait Instruction: Debug {
    // fn fetch_operands(&mut self, reader: &mut BytecodeReader);
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // Nothing to do
    }
    
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()>;
}
