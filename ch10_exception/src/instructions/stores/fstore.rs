#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Store float into local variable
#[derive(Default, Debug)]
pub struct FSTORE {
    pub index: usize,
}

impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fstore(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_0;

impl Instruction for FSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fstore(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_1;

impl Instruction for FSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fstore(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_2;

impl Instruction for FSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fstore(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FSTORE_3;

impl Instruction for FSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fstore(frame, 3);

        Ok(())
    }
}

fn _fstore(frame: &mut Frame, index: usize) {
    let val = frame.get_operand_stack().pop_float();
    frame.get_local_vars().set_float(index, val);
}
