#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::branch;

/// Branch if int comparison succeeds
#[derive(Default, Debug)]
pub struct IF_ICMPEQ {
    pub offset: i64,
}

impl Instruction for IF_ICMPEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (val1, val2)= _icmp_pop(frame);
        if val1 == val2 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IF_ICMPNE {
    pub offset: i64,
}

impl Instruction for IF_ICMPNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (val1, val2)= _icmp_pop(frame);
        if val1 != val2 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IF_ICMPLT {
    pub offset: i64,
}

impl Instruction for IF_ICMPLT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (val1, val2)= _icmp_pop(frame);
        if val1 < val2 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IF_ICMPLE {
    pub offset: i64,
}

impl Instruction for IF_ICMPLE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (val1, val2)= _icmp_pop(frame);
        if val1 <= val2 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IF_ICMPGT {
    pub offset: i64,
}

impl Instruction for IF_ICMPGT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (val1, val2)= _icmp_pop(frame);
        if val1 > val2 {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IF_ICMPGE {
    pub offset: i64,
}

impl Instruction for IF_ICMPGE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (val1, val2)= _icmp_pop(frame);
        if val1 >= val2 {
            branch(frame, self.offset);
        }
    }
}

fn _icmp_pop(frame: &mut Frame) -> (i32, i32) {
    let stack = frame.operand_stack_mut();
    let val2 = stack.pop_int();
    let val1 = stack.pop_int();
    (val1, val2)
}
