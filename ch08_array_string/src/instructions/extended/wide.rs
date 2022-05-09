#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::loads::*;
use super::super::math::*;
use super::super::stores::*;

/// Extend local variable index by additional bytes
#[derive(Default, Debug)]
pub struct WIDE {
    modified_instruction: Option<Box<dyn Instruction>>,
}

impl Instruction for WIDE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        let opcode = reader.read_u8();
        self.modified_instruction = match opcode {
            0x15 => {
                let mut inst = ILOAD::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x16 => {
                let mut inst = LLOAD::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x17 => {
                let mut inst = FLOAD::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x18 => {
                let mut inst = DLOAD::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x19 => {
                let mut inst = ALOAD::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x36 => {
                let mut inst = ISTORE::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x37 => {
                let mut inst = LSTORE::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x38 => {
                let mut inst = FSTORE::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x39 => {
                let mut inst = DSTORE::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x3a => {
                let mut inst = ASTORE::default();
                inst.index = reader.read_u16() as usize;
                Some(Box::new(inst))
            },
            0x84 => {
                let mut inst = IINC::default();
                inst.index = reader.read_u16() as usize;
                inst._const = reader.read_i16() as i32;
                Some(Box::new(inst))
            },
            0xa9 => {
                panic!("{}", "Unsupported opcode: 0xa9!");
            },
            _ => {
                panic!("Unsupported opcode: 0x{:x}!", opcode);
            }
        }
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        self.modified_instruction.as_mut().unwrap().execute(frame)?;

        Ok(())
    }
}