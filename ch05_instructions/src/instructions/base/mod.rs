pub mod bytecode_reader;
pub mod instruction;

use super::super::rtda::Frame;

pub fn branch(frame: &mut Frame, offset: i32) {
	let pc = frame.get_thread().borrow().pc();
	let next_pc = pc + offset;
	frame.set_next_pc(next_pc);
}
