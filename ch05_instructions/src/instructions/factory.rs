use super::consts::ICONST_0;
use super::instruction::Instruction;
use super::ipush::*;
use super::nop::NOP;

pub fn new_instruction(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x00 => {
            Box::new(NOP::default())
        },
        0x03 => {
            Box::new(ICONST_0::default())
        },
        0x10 => {
            Box::new(BIPUSH::default())
        },
        0x11 => {
            Box::new(SIPUSH::default())
        },
        _ => {
            panic!("Unsupported opcode: 0x{:x}!", opcode)
        }
    }
}
