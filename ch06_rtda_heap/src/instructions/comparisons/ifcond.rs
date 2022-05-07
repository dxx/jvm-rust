#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::branch;

/// Branch if int comparison with zero succeeds
#[derive(Default, Debug)]
pub struct IFEQ {
    pub offset: i64,
}

impl Instruction for IFEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val == 0 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IFNE {
    pub offset: i64,
}

impl Instruction for IFNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val != 0 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IFLT {
    pub offset: i64,
}

impl Instruction for IFLT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val < 0 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IFLE {
    pub offset: i64,
}

impl Instruction for IFLE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val <= 0 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IFGT {
    pub offset: i64,
}

impl Instruction for IFGT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val > 0 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IFGE {
    pub offset: i64,
}

impl Instruction for IFGE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val >= 0 {
            branch(frame, self.offset);
        }
    }
}
