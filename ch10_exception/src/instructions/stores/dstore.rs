#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Store double into local variable
#[derive(Default, Debug)]
pub struct DSTORE {
    pub index: usize,
}

impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _dstore(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DSTORE_0;

impl Instruction for DSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _dstore(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DSTORE_1;

impl Instruction for DSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _dstore(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DSTORE_2;

impl Instruction for DSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _dstore(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DSTORE_3;

impl Instruction for DSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _dstore(frame, 3);

        Ok(())
    }
}

fn _dstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack_mut().pop_double();
    frame.local_vars_mut().set_double(index, val);
}
