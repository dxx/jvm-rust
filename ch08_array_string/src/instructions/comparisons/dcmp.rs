#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Instruction;

/// Compare double
#[derive(Default, Debug)]
pub struct DCMPG;

impl Instruction for DCMPG {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _dcmp(frame, true);

		Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DCMPL;

impl Instruction for DCMPL {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        _dcmp(frame, false);

		Ok(())
    }
}

fn _dcmp(frame: &mut Frame, g_flag: bool) {
	let stack = frame.operand_stack_mut();
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
