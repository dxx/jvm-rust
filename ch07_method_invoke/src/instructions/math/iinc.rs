#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Increment local variable by constant
#[derive(Default, Debug)]
pub struct IINC {
    pub index: usize,
    pub _const: i32,
}

impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self._const = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let local_vars = frame.local_vars_mut();
        let val = local_vars.get_int(self.index);
        let val = val + self._const;
        local_vars.set_int(self.index, val);
    }
}
