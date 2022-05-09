#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Load int from local variable
#[derive(Default, Debug)]
pub struct ILOAD {
    pub index: usize,
}

impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _iload(frame, self.index);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_0;

impl Instruction for ILOAD_0 {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _iload(frame, 0);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_1;

impl Instruction for ILOAD_1 {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _iload(frame, 1);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_2;

impl Instruction for ILOAD_2 {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _iload(frame, 2);

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_3;

impl Instruction for ILOAD_3 {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _iload(frame, 3);

        Ok(())
    }
}

fn _iload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars_mut().get_int(index);
    frame.operand_stack_mut().push_int(val);
}
