#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_fieldref::FieldRef;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;

/// Set static field in class
#[derive(Default, Debug)]
pub struct PUT_STATIC {
    index: u64, 
}

impl Instruction for PUT_STATIC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let field = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<FieldRef>().unwrap().resolved_field(current_class.clone());
        
        let class = field.borrow().get_class().unwrap();

        // TODO: init class

        if !field.borrow().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.borrow().is_final() {
            if current_class.ne(&class) || current_method.borrow().name() != "<clinit>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = field.borrow().descriptor();
        let slot_id = field.borrow().slot_id() as usize;
        let slots = class.borrow_mut().static_vars();
        let stack = frame.operand_stack_mut();

        if descriptor.starts_with("Z") || descriptor.starts_with("B") || descriptor.starts_with("C") ||
            descriptor.starts_with("S") || descriptor.starts_with("I") {
            slots.borrow_mut().set_int(slot_id, stack.pop_int());
        } else if descriptor.starts_with("F") {
            slots.borrow_mut().set_float(slot_id, stack.pop_float());
        } else if descriptor.starts_with("J") {
            slots.borrow_mut().set_long(slot_id, stack.pop_long());
        } else if descriptor.starts_with("D") {
            slots.borrow_mut().set_double(slot_id, stack.pop_double());
        } else if descriptor.starts_with("L") || descriptor.starts_with("[") {
            slots.borrow_mut().set_ref(slot_id, stack.pop_ref());
        } else {
            // TODO
        }
    }
}
