use std::result::Result;
use super::instruction::Instruction;
use super::comparisons::*;
use super::constants::*;
use super::control::*;
use super::conversions::*;
use super::extended::*;
use super::loads::*;
use super::math::*;
use super::stack::*;
use super::stores::*;

pub fn new_instruction(opcode: u8) -> Result<Box<dyn Instruction>, String> {
    let inst: Box<dyn Instruction> = match opcode {
        0x00 => {
            Box::new(NOP::default())
        },
        0x01 => {
            Box::new(ACONST_NULL::default())
        },
        0x02 => {
            Box::new(ICONST_M1::default())
        },
        0x03 => {
            Box::new(ICONST_0::default())
        },
        0x04 => {
            Box::new(ICONST_1::default())
        },
        0x05 => {
            Box::new(ICONST_2::default())
        },
        0x06 => {
            Box::new(ICONST_3::default())
        },
        0x07 => {
            Box::new(ICONST_4::default())
        },
        0x08 => {
            Box::new(ICONST_5::default())
        },
        0x09 => {
            Box::new(LCONST_0::default())
        },
        0x0a => {
            Box::new(LCONST_1::default())
        },
        0x0b => {
            Box::new(FCONST_0::default())
        },
        0x0c => {
            Box::new(FCONST_1::default())
        },
        0x0d => {
            Box::new(FCONST_2::default())
        },
        0x0e => {
            Box::new(DCONST_0::default())
        },
        0x0f => {
            Box::new(DCONST_1::default())
        },
        0x10 => {
            Box::new(BIPUSH::default())
        },
        0x11 => {
            Box::new(SIPUSH::default())
        },
        // 0x12 => {
        //     Box::new(LDC::default())
        // },
        // 0x13 => {
        //     Box::new(LDC_W::default())
        // },
        // 0x14 => {
        //     Box::new(LDC2_W::default())
        // },
        0x15 => {
            Box::new(SIPUSH::default())
        },
        _ => {
            return Err(String::from(format!("Unsupported opcode: 0x{:x}!", opcode)));
        }
    };

    Ok(inst)
}
