#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Store reference into local variable
#[derive(Default, Debug)]
pub struct ASTORE {
    pub index: usize,
}

impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _astore(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_0;

impl Instruction for ASTORE_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _astore(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_1;

impl Instruction for ASTORE_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _astore(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_2;

impl Instruction for ASTORE_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _astore(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_3;

impl Instruction for ASTORE_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _astore(frame, 3);

        Ok(())
    }
}

fn _astore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack_mut().pop_ref();
    frame.local_vars_mut().set_ref(index, val);
}
