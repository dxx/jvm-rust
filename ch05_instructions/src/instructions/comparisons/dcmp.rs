#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Compare double
#[derive(Default, Debug)]
pub struct DCMPG;

impl Instruction for DCMPG {
    fn execute(&self, frame: &mut Frame) {
        _dcmp(frame, true);
    }
}

#[derive(Default, Debug)]
pub struct DCMPL;

impl Instruction for DCMPL {
    fn execute(&self, frame: &mut Frame) {
        _dcmp(frame, false);
    }
}

fn _dcmp(frame: &mut Frame, g_flag: bool) {
	let stack = frame.get_operand_stack();
	let v2 = stack.pop_double();
	let v1 = stack.pop_double();
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
