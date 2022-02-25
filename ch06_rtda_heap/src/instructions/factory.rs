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
            Box::new(ILOAD::default())
        },
        0x16 => {
            Box::new(LLOAD::default())
        },
        0x17 => {
            Box::new(FLOAD::default())
        },
        0x18 => {
            Box::new(DLOAD::default())
        },
        0x19 => {
            Box::new(ALOAD::default())
        },
        0x1a => {
            Box::new(ILOAD_0::default())
        },
        0x1b => {
            Box::new(ILOAD_1::default())
        },
        0x1c => {
            Box::new(ILOAD_2::default())
        },
        0x1d => {
            Box::new(ILOAD_3::default())
        },
        0x1e => {
            Box::new(LLOAD_0::default())
        },
        0x1f => {
            Box::new(LLOAD_1::default())
        },
        0x20 => {
            Box::new(LLOAD_2::default())
        },
        0x21 => {
            Box::new(LLOAD_3::default())
        },
        0x22 => {
            Box::new(FLOAD_0::default())
        },
        0x23 => {
            Box::new(FLOAD_1::default())
        },
        0x24 => {
            Box::new(FLOAD_2::default())
        },
        0x25 => {
            Box::new(FLOAD_3::default())
        },
        0x26 => {
            Box::new(DLOAD_0::default())
        },
        0x27 => {
            Box::new(DLOAD_1::default())
        },
        0x28 => {
            Box::new(DLOAD_2::default())
        },
        0x29 => {
            Box::new(DLOAD_3::default())
        },
        0x2a => {
            Box::new(ALOAD_0::default())
        },
        0x2b => {
            Box::new(ALOAD_1::default())
        },
        0x2c => {
            Box::new(ALOAD_2::default())
        },
        0x2d => {
            Box::new(ALOAD_3::default())
        },
        // 0x2e => {
        //     Box::new(IALOAD::default())
        // },
        // 0x2f => {
        //     Box::new(LALOAD::default())
        // },
        // 0x30 => {
        //     Box::new(FALOAD::default())
        // },
        // 0x31 => {
        //     Box::new(DALOAD::default())
        // },
        // 0x32 => {
        //     Box::new(AALOAD::default())
        // },
        // 0x33 => {
        //     Box::new(BALOAD::default())
        // },
        // 0x34 => {
        //     Box::new(CALOAD::default())
        // },
        // 0x35 => {
        //     Box::new(SALOAD::default())
        // },
        0x36 => {
            Box::new(ISTORE::default())
        },
        0x37 => {
            Box::new(LSTORE::default())
        },
        0x38 => {
            Box::new(FSTORE::default())
        },
        0x39 => {
            Box::new(DSTORE::default())
        },
        0x3a => {
            Box::new(ASTORE::default())
        },
        0x3b => {
            Box::new(ISTORE_0::default())
        },
        0x3c => {
            Box::new(ISTORE_1::default())
        },
        0x3d => {
            Box::new(ISTORE_2::default())
        },
        0x3e => {
            Box::new(ISTORE_3::default())
        },
        0x3f => {
            Box::new(LSTORE_0::default())
        },
        0x40 => {
            Box::new(LSTORE_1::default())
        },
        0x41 => {
            Box::new(LSTORE_2::default())
        },
        0x42 => {
            Box::new(LSTORE_3::default())
        },
        0x43 => {
            Box::new(FSTORE_0::default())
        },
        0x44 => {
            Box::new(FSTORE_1::default())
        },
        0x45 => {
            Box::new(FSTORE_2::default())
        },
        0x46 => {
            Box::new(FSTORE_3::default())
        },
        0x47 => {
            Box::new(DSTORE_0::default())
        },
        0x48 => {
            Box::new(DSTORE_1::default())
        },
        0x49 => {
            Box::new(DSTORE_2::default())
        },
        0x4a => {
            Box::new(DSTORE_3::default())
        },
        0x4b => {
            Box::new(ASTORE_0::default())
        },
        0x4c => {
            Box::new(ASTORE_1::default())
        },
        0x4d => {
            Box::new(ASTORE_2::default())
        },
        0x4e => {
            Box::new(ASTORE_3::default())
        },
        // 0x4f => {
        //     Box::new(IASTORE::default())
        // },
        // 0x50 => {
        //     Box::new(LASTORE::default())
        // },
        // 0x51 => {
        //     Box::new(FASTORE::default())
        // },
        // 0x52 => {
        //     Box::new(DASTORE::default())
        // },
        // 0x53 => {
        //     Box::new(AASTORE::default())
        // },
        // 0x54 => {
        //     Box::new(BASTORE::default())
        // },
        // 0x55 => {
        //     Box::new(CASTORE::default())
        // },
        // 0x56 => {
        //     Box::new(SASTORE::default())
        // },
        0x57 => {
            Box::new(POP::default())
        },
        0x58 => {
            Box::new(POP2::default())
        },
        0x59 => {
            Box::new(DUP::default())
        },
        0x5a => {
            Box::new(DUP_X1::default())
        },
        0x5b => {
            Box::new(DUP_X2::default())
        },
        0x5c => {
            Box::new(DUP2::default())
        },
        0x5d => {
            Box::new(DUP2_X1::default())
        },
        0x5e => {
            Box::new(DUP2_X2::default())
        },
        0x5f => {
            Box::new(SWAP::default())
        },
        0x60 => {
            Box::new(IADD::default())
        },
        0x61 => {
            Box::new(LADD::default())
        },
        0x62 => {
            Box::new(FADD::default())
        },
        0x63 => {
            Box::new(DADD::default())
        },
        0x64 => {
            Box::new(ISUB::default())
        },
        0x65 => {
            Box::new(LSUB::default())
        },
        0x66 => {
            Box::new(FSUB::default())
        },
        0x67 => {
            Box::new(DSUB::default())
        },
        0x68 => {
            Box::new(IMUL::default())
        },
        0x69 => {
            Box::new(LMUL::default())
        },
        0x6a => {
            Box::new(FMUL::default())
        },
        0x6b => {
            Box::new(DMUL::default())
        },
        0x6c => {
            Box::new(IDIV::default())
        },
        0x6d => {
            Box::new(LDIV::default())
        },
        0x6e => {
            Box::new(FDIV::default())
        },
        0x6f => {
            Box::new(DDIV::default())
        },
        0x70 => {
            Box::new(IREM::default())
        },
        0x71 => {
            Box::new(LREM::default())
        },
        0x72 => {
            Box::new(FREM::default())
        },
        0x73 => {
            Box::new(DREM::default())
        },
        0x74 => {
            Box::new(INEG::default())
        },
        0x75 => {
            Box::new(LNEG::default())
        },
        0x76 => {
            Box::new(FNEG::default())
        },
        0x77 => {
            Box::new(DNEG::default())
        },
        0x78 => {
            Box::new(ISHL::default())
        },
        0x79 => {
            Box::new(LSHL::default())
        },
        0x7a => {
            Box::new(ISHR::default())
        },
        0x7b => {
            Box::new(LSHR::default())
        },
        0x7c => {
            Box::new(IUSHR::default())
        },
        0x7d => {
            Box::new(LUSHR::default())
        },
        0x7e => {
            Box::new(IAND::default())
        },
        0x7f => {
            Box::new(LAND::default())
        },
        0x80 => {
            Box::new(IOR::default())
        },
        0x81 => {
            Box::new(LOR::default())
        },
        0x82 => {
            Box::new(IXOR::default())
        },
        0x83 => {
            Box::new(LXOR::default())
        },
        0x84 => {
            Box::new(IINC::default())
        },
        0x85 => {
            Box::new(I2L::default())
        },
        0x86 => {
            Box::new(I2F::default())
        },
        0x87 => {
            Box::new(I2D::default())
        },
        0x88 => {
            Box::new(L2I::default())
        },
        0x89 => {
            Box::new(L2F::default())
        },
        0x8a => {
            Box::new(L2D::default())
        },
        0x8b => {
            Box::new(F2I::default())
        },
        0x8c => {
            Box::new(F2L::default())
        },
        0x8d => {
            Box::new(F2D::default())
        },
        0x8e => {
            Box::new(D2I::default())
        },
        0x8f => {
            Box::new(D2L::default())
        },
        0x90 => {
            Box::new(D2F::default())
        },
        0x91 => {
            Box::new(I2B::default())
        },
        0x92 => {
            Box::new(I2C::default())
        },
        0x93 => {
            Box::new(I2S::default())
        },
        0x94 => {
            Box::new(LCMP::default())
        },
        0x95 => {
            Box::new(FCMPL::default())
        },
        0x96 => {
            Box::new(FCMPG::default())
        },
        0x97 => {
            Box::new(DCMPL::default())
        },
        0x98 => {
            Box::new(DCMPG::default())
        },
        0x99 => {
            Box::new(IFEQ::default())
        },
        0x9a => {
            Box::new(IFNE::default())
        },
        0x9b => {
            Box::new(IFLT::default())
        },
        0x9c => {
            Box::new(IFGE::default())
        },
        0x9d => {
            Box::new(IFGT::default())
        },
        0x9e => {
            Box::new(IFLE::default())
        },
        0x9f => {
            Box::new(IF_ICMPEQ::default())
        },
        0xa0 => {
            Box::new(IF_ICMPNE::default())
        },
        0xa1 => {
            Box::new(IF_ICMPLT::default())
        },
        0xa2 => {
            Box::new(IF_ICMPGE::default())
        },
        0xa3 => {
            Box::new(IF_ICMPGT::default())
        },
        0xa4 => {
            Box::new(IF_ICMPLE::default())
        },
        0xa5 => {
            Box::new(IF_ACMPEQ::default())
        },
        0xa6 => {
            Box::new(IF_ACMPNE::default())
        },
        0xa7 => {
            Box::new(GOTO::default())
        },
        // 0xa8 => {
        //     Box::new(JSR::default())
        // },
        // 0xa9 => {
        //     Box::new(RET::default())
        // },
        0xaa => {
            Box::new(TABLE_SWITCH::default())
        },
        0xab => {
            Box::new(LOOKUP_SWITCH::default())
        },
        // 0xac => {
        //     Box::new(IRETURN::default())
        // },
        // 0xad => {
        //     Box::new(LRETURN::default())
        // },
        // 0xae => {
        //     Box::new(FRETURN::default())
        // },
        // 0xaf => {
        //     Box::new(DRETURN::default())
        // },
        // 0xb0 => {
        //     Box::new(ARETURN::default())
        // },
        // 0xb1 => {
        //     Box::new(_RETURN::default())
        // },
        // 0xb2 => {
        //     Box::new(GET_STATIC::default())
        // },
        // 0xb3 => {
        //     Box::new(PUT_STATIC::default())
        // },
        // 0xb4 => {
        //     Box::new(GET_FIELD::default())
        // },
        // 0xb5 => {
        //     Box::new(PUT_FIELD::default())
        // },
        // 0xb6 => {
        //     Box::new(INVOKE_VIRTUAL::default())
        // },
        // 0xb7 => {
        //     Box::new(INVOKE_SPECIAL::default())
        // },
        // 0xb8 => {
        //     Box::new(INVOKE_STATIC::default())
        // },
        // 0xb9 => {
        //     Box::new(INVOKE_INTERFACE::default())
        // },
        // 0xba => {
        //     Box::new(INVOKE_DYNAMIC::default())
        // },
        // 0xbb => {
        //     Box::new(NEW::default())
        // },
        // 0xbc => {
        //     Box::new(NEW_ARRAY::default())
        // },
        // 0xbd => {
        //     Box::new(ANEW_ARRAY::default())
        // },
        // 0xbe => {
        //     Box::new(ARRAYLENG::default())
        // },
        // 0xbf => {
        //     Box::new(ATHROW::default())
        // },
        // 0xc0 => {
        //     Box::new(CHECK_CAST::default())
        // },
        // 0xc1 => {
        //     Box::new(INSTANCE_OF::default())
        // },
        // 0xc2 => {
        //     Box::new(MONITORENTER::default())
        // },
        // 0xc3 => {
        //     Box::new(MONITOREXIT::default())
        // },
        0xc4 => {
            Box::new(WIDE::default())
        },
        // 0xc5 => {
        //     Box::new(MULTI_ANEW_ARRAY::default())
        // },
        0xc6 => {
            Box::new(IFNULL::default())
        },
        0xc7 => {
            Box::new(IFNONNULL::default())
        },
        0xc8 => {
            Box::new(GOTO_W::default())
        },
        // 0xc9 => {
        //     Box::new(JSR_W::default())
        // },
        // 0xca => {
        //     breakpoint
        // },
        // 0xfe => {
        //     impdep1
        // },
        // 0xff => {
        //     impdep2
        // },
        _ => {
            return Err(String::from(format!("Unsupported opcode: 0x{:x}!", opcode)));
        }
    };

    Ok(inst)
}
