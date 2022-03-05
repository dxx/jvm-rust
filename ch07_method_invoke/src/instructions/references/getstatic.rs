#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_fieldref::FieldRef;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Get static field from class
#[derive(Default, Debug)]
pub struct GET_STATIC {
    index: u64, 
}

impl Instruction for GET_STATIC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.get_method();
        let current_class = current_method.borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let field = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<FieldRef>().unwrap().resolved_field(current_class.clone());
        
        let class = field.borrow().get_class().unwrap();

        // TODO: init class

        if !field.borrow().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let descriptor = field.borrow().descriptor().as_bytes()[0];
        let slot_id = field.borrow().slot_id() as usize;
        let slots = class.borrow_mut().static_vars();
        let stack = frame.get_operand_stack();

        match descriptor {
            b'Z' | b'B' | b'C' | b'S' | b'I' => {
                stack.push_int(slots.borrow().get_int(slot_id));
            },
            b'F' => {
                stack.push_float(slots.borrow().get_float(slot_id));
            },
            b'J' => {
                stack.push_long(slots.borrow().get_long(slot_id));
            },
            b'D' => {
                stack.push_double(slots.borrow().get_double(slot_id));
            },
            b'L' | b'[' => {
                stack.push_ref(slots.borrow().get_ref(slot_id));
            },
            _ => {
                // TODO
            }
        }
    }
}
