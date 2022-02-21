#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Compare float
#[derive(Default, Debug)]
pub struct FCMPG;

impl Instruction for FCMPG {
    fn execute(&self, frame: &mut Frame) {
        _fcmp(frame, true);
    }
}

#[derive(Default, Debug)]
pub struct FCMPL;

impl Instruction for FCMPL {
    fn execute(&self, frame: &mut Frame) {
        _fcmp(frame, false);
    }
}

fn _fcmp(frame: &mut Frame, g_flag: bool) {
	let stack = frame.get_operand_stack();
	let v2 = stack.pop_float();
	let v1 = stack.pop_float();
	if v1 > v2 {
		stack.push_int(1);
	} else if v1 == v2 {
		stack.push_int(0);
	} else if v1 < v2 {
		stack.push_int(-1);
	} else if g_flag {
		stack.push_int(1);
	} else {
		stack.push_int(-1);
	}
}
