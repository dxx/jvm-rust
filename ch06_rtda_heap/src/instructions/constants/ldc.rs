#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::classfile::constant_pool;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Push item from run-time constant pool
#[derive(Default, Debug)]
pub struct LDC {
    index: u64,
}

impl Instruction for LDC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _ldc(frame, self.index);
    }
}

/// Push item from run-time constant pool (wide index)
#[derive(Default, Debug)]
pub struct LDC_W {
    index: u64,
}

impl Instruction for LDC_W {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _ldc(frame, self.index);
    }
}

/// Push long or double from run-time constant pool (wide index)
#[derive(Default, Debug)]
pub struct LDC2_W {
    index: u64,
}

impl Instruction for LDC2_W {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let index = self.index;
        let method = frame.get_method();
        let stack = frame.get_operand_stack();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let tag = r_cp.borrow().get_constant(index as usize).tag();
    
        match tag {
            constant_pool::CONSTANT_LONG => {
                let val = *r_cp.borrow().get_constant(index as usize)
                    .as_any().downcast_ref::<i64>().unwrap();
                stack.push_long(val);
            },
            constant_pool::CONSTANT_DOUBLE => {
                let val = *r_cp.borrow().get_constant(index as usize)
                    .as_any().downcast_ref::<f64>().unwrap();
                stack.push_double(val);
            },
            _ => {
                panic!("java.lang.ClassFormatError");
            }
        }
    }
}

fn _ldc(frame: &mut Frame, index: u64) {
    let method = frame.get_method();
    let stack = frame.get_operand_stack();
    let current_class = method.borrow().get_class();
    let r_cp = current_class.borrow_mut().constant_pool();
    let tag = r_cp.borrow().get_constant(index as usize).tag();

    match tag {
        constant_pool::CONSTANT_INTEGER => {
            let val = *r_cp.borrow().get_constant(index as usize)
                .as_any().downcast_ref::<i32>().unwrap();
            stack.push_int(val);
        },
        constant_pool::CONSTANT_FLOAT => {
            let val = *r_cp.borrow().get_constant(index as usize)
                .as_any().downcast_ref::<f32>().unwrap();
            stack.push_float(val);
        },
        _ => {
            panic!("TODO: ldc!");
        }
    }
}

