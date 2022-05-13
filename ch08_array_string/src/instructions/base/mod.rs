pub mod bytecode_reader;
pub mod instruction;

use crate::types::RcRefCell;
use crate::rtda::Frame;
use crate::rtda::Thread;
use crate::rtda::method::Method;
use crate::rtda::class::Class;

pub fn branch(frame: &mut Frame, offset: i64) {
	let pc = frame.thread().borrow().pc();
	let next_pc = pc + offset;
	frame.set_next_pc(next_pc);
}

pub fn invoke_method(frame: &mut Frame, method: &RcRefCell<Method>) {
	let thread = frame.thread();
	let mut new_frame = thread.borrow_mut().new_frame(thread.clone(), method.clone());

	let arg_slot_count = method.borrow().arg_slot_count() as i32;
	if arg_slot_count > 0 {
		let mut i = arg_slot_count - 1;
		while i >= 0 {
			let slot = frame.operand_stack_mut().pop_slot();
			new_frame.local_vars_mut().set_slot(i as usize, slot);
			i -= 1;
		}
	}

	thread.borrow_mut().push_frame(new_frame);

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

pub fn init_class(thread: &RcRefCell<Thread>, class: &RcRefCell<Class>) {
	class.borrow_mut().start_init();
	schedule_clinit(thread, class);
	init_super_class(thread, class);
}

pub fn schedule_clinit(thread: &RcRefCell<Thread>, class: &RcRefCell<Class>) {
	let clinit = class.borrow().get_clinit_method();
	if clinit.is_some() {
		// exec <clinit>
		let new_frame = thread.borrow_mut().new_frame(thread.clone(), clinit.unwrap());
		thread.borrow_mut().push_frame(new_frame);
	}
}

pub fn init_super_class(thread: &RcRefCell<Thread>, class: &RcRefCell<Class>) {
	if !class.borrow().is_interface() {
		let super_class = class.borrow().super_class();
		if super_class.is_some() && !super_class.as_ref().unwrap().borrow().init_started() {
			init_class(thread, super_class.as_ref().unwrap());
		}
	}
}
