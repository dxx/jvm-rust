pub mod bytecode_reader;
pub mod instruction;

use crate::rtda::Frame;
use crate::rtda::method::Method;
use std::rc::Rc;
use std::cell::RefCell;

pub fn branch(frame: &mut Frame, offset: i64) {
	let pc = frame.get_thread().borrow().pc();
	let next_pc = pc + offset;
	frame.set_next_pc(next_pc);
}

pub fn invoke_method(frame: &mut Frame, method: &Rc<RefCell<Method>>) {
	let thread = frame.get_thread();
	let mut new_frame = thread.borrow_mut().new_frame(thread.clone(), method.clone());

	let arg_slot_count = method.borrow().arg_slot_count() as i32;
	if arg_slot_count > 0 {
		let mut i = arg_slot_count - 1;
		while i >= 0 {
			let slot = frame.get_operand_stack().pop_slot();
			new_frame.get_local_vars().set_slot(i as usize, slot);
			i -= 1;
		}
	}

	// Hack!
	if method.borrow().is_native() {
		if method.borrow().name() == "registerNatives" {
			thread.borrow_mut().pop_frame();
		} else {
			panic!("native method: {}.{}{}",
				method.borrow().get_class().borrow().name(),
				method.borrow().name(),
				method.borrow().descriptor()
			);
		}
	}
}
