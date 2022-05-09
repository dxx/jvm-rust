#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::branch;

/// Branch if reference comparison succeeds
#[derive(Default, Debug)]
pub struct IF_ACMPEQ {
    pub offset: i64,
}

impl Instruction for IF_ACMPEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        if _acmp(frame) {
            branch(frame, self.offset);
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct IF_ACMPNE {
    pub offset: i64,
}

impl Instruction for IF_ACMPNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        if !_acmp(frame) {
            branch(frame, self.offset);
        }

        Ok(())
    }
}

fn _acmp(frame: &mut Frame) -> bool {
    let stack = frame.operand_stack_mut();
    let ref2 = stack.pop_ref();
    let ref1 = stack.pop_ref();
    ref1 == ref2
}
