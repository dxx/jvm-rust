#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;
use super::super::branch;

/// Access jump table by index and jump
/// tableswitch
/// <0-3 byte pad>
/// defaultbyte1
/// defaultbyte2
/// defaultbyte3
/// defaultbyte4
/// lowbyte1
/// lowbyte2
/// lowbyte3
/// lowbyte4
/// highbyte1
/// highbyte2
/// highbyte3
/// highbyte4
/// jump offsets...
#[derive(Default, Debug)]
pub struct TABLE_SWITCH {
    default_offset: i32,
    low: i32,
    high: i32,
    jump_offsets: Vec<i32>,
}

impl Instruction for TABLE_SWITCH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32();
        self.low = reader.read_i32();
        self.high = reader.read_i32();
        let jump_offset_count = self.high - self.low + 1;
        self.jump_offsets = reader.read_i32s(jump_offset_count);
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let index = frame.operand_stack_mut().pop_int();
        let offset = 
        if index >= self.low && index <= self.high {
            self.jump_offsets[(index - self.low) as usize]
        } else {
            self.default_offset
        };
        branch(frame, offset as i64);

        Ok(())
    }
}
