#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_classref::ClassRef;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;
use super::super::init_class;
use std::rc::Rc;
use std::cell::RefCell;

/// Create new object
#[derive(Default, Debug)]
pub struct NEW {
    index: u64, 
}

impl Instruction for NEW {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let method = frame.get_method();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let class = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<ClassRef>().unwrap().resolved_class(current_class);

        if !class.borrow().init_started() {
            frame.revert_next_pc();
            init_class(&frame.get_thread(), &class);
            return Ok(());
        }

        if class.borrow().is_interface() || class.borrow().is_abstract() {
            return Err("java.lang.InstantiationError".into());
        }
        let _ref = class.borrow().new_object(class.clone());
        frame.get_operand_stack().push_ref(Some(Rc::new(RefCell::new(_ref))));

        Ok(())
    }
}
