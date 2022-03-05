#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Invoke instance method;
/// Special handling for superclass, private, and instance initialization method invocations
#[derive(Default, Debug)]
pub struct INVOKE_SPECIAL {
    index: u64,
}

impl Instruction for INVOKE_SPECIAL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    /// Hack!
    fn execute(&mut self, frame: &mut Frame) {
        frame.get_operand_stack().pop_ref();
    }
}
