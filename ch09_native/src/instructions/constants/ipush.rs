#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Push byte
#[derive(Default, Debug)]
pub struct BIPUSH {
    val: i8,
}

impl Instruction for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i8();
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        frame.operand_stack_mut().push_int(self.val as i32);

        Ok(())
    }
}

/// Push short
#[derive(Default, Debug)]
pub struct SIPUSH {
    val: i16,
}

impl Instruction for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i16();
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        frame.operand_stack_mut().push_int(self.val as i32);

        Ok(())
    }
}
