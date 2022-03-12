#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;

/// Load reference from local variable
#[derive(Default, Debug)]
pub struct ALOAD {
    pub index: usize,
}

impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _aload(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_0;

impl Instruction for ALOAD_0 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _aload(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_1;

impl Instruction for ALOAD_1 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _aload(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_2;

impl Instruction for ALOAD_2 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _aload(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_3;

impl Instruction for ALOAD_3 {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        _aload(frame, 3);

        Ok(())
    }
}

fn _aload(frame: &mut Frame, index: usize) {
    let val = frame.get_local_vars().get_ref(index);
    frame.get_operand_stack().push_ref(val);
}
