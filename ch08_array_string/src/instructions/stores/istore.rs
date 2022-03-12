#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Store int into local variable
#[derive(Default, Debug)]
pub struct ISTORE {
    pub index: usize,
}

impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _istore(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_0;

impl Instruction for ISTORE_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _istore(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_1;

impl Instruction for ISTORE_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _istore(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_2;

impl Instruction for ISTORE_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _istore(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_3;

impl Instruction for ISTORE_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _istore(frame, 3);

        Ok(())
    }
}

fn _istore(frame: &mut Frame, index: usize) {
    let val = frame.get_operand_stack().pop_int();
    frame.get_local_vars().set_int(index, val);
}
