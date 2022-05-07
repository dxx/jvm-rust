#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_fieldref::FieldRef;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Set field in object
#[derive(Default, Debug)]
pub struct PUT_FIELD {
    index: u64, 
}

impl Instruction for PUT_FIELD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let field = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<FieldRef>().unwrap().resolved_field(current_class.clone());
        
        let class = field.borrow().get_class();

        if field.borrow().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.borrow().is_final() {
            if current_class.ne(&class) || current_method.borrow().name() != "<init>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = field.borrow().descriptor().as_bytes()[0];
        let slot_id = field.borrow().slot_id() as usize;
        let stack = frame.operand_stack_mut();

        match descriptor {
            b'Z' | b'B' | b'C' | b'S' | b'I' => {
                let val = stack.pop_int();
                let _ref = stack.pop_ref();
                if _ref.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                let obj = _ref.unwrap();
                obj.borrow_mut().fields_mut().set_int(slot_id, val);
            },
            b'F' => {
                let val = stack.pop_float();
                let _ref = stack.pop_ref();
                if _ref.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                let obj = _ref.unwrap();
                obj.borrow_mut().fields_mut().set_float(slot_id, val);
            },
            b'J' => {
                let val = stack.pop_long();
                let _ref = stack.pop_ref();
                if _ref.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                let obj = _ref.unwrap();
                obj.borrow_mut().fields_mut().set_long(slot_id, val);
            },
            b'D' => {
                let val = stack.pop_double();
                let _ref = stack.pop_ref();
                if _ref.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                let obj = _ref.unwrap();
                obj.borrow_mut().fields_mut().set_double(slot_id, val);
            },
            b'L' | b'[' => {
                let val = stack.pop_ref();
                let _ref = stack.pop_ref();
                if _ref.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                let obj = _ref.unwrap();
                obj.borrow_mut().fields_mut().set_ref(slot_id, val);
            },
            _ => {
                // TODO
            }
        }
    }
}
