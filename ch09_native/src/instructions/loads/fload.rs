#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Load float from local variable
#[derive(Default, Debug)]
pub struct FLOAD {
    pub index: usize,
}

impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fload(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_0;

impl Instruction for FLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fload(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_1;

impl Instruction for FLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fload(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_2;

impl Instruction for FLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fload(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_3;

impl Instruction for FLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _fload(frame, 3);

        Ok(())
    }
}

fn _fload(frame: &mut Frame, index: usize) {
    let val = frame.get_local_vars().get_float(index);
    frame.get_operand_stack().push_float(val);
}
