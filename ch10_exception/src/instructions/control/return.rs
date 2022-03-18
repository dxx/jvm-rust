#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use super::super::instruction::Result;
use super::super::instruction::Instruction;

/// Return void from method
#[derive(Default, Debug)]
pub struct RETURN;

impl Instruction for RETURN {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        frame.get_thread().borrow_mut().pop_frame();

        Ok(())
    }
}

/// Return reference from method
#[derive(Default, Debug)]
pub struct ARETURN;

impl Instruction for ARETURN {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let thread = frame.get_thread();
        let _current_frame = thread.borrow_mut().pop_frame();
        let val = frame.get_operand_stack().pop_ref();
        thread.borrow_mut().top_frame().borrow_mut().get_operand_stack().push_ref(val);

        Ok(())
    }
}

/// Return double from method
#[derive(Default, Debug)]
pub struct DRETURN;

impl Instruction for DRETURN {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let thread = frame.get_thread();
        let _current_frame = thread.borrow_mut().pop_frame();
        let val = frame.get_operand_stack().pop_double();
        thread.borrow_mut().top_frame().borrow_mut().get_operand_stack().push_double(val);

        Ok(())
    }
}

/// Return float from method
#[derive(Default, Debug)]
pub struct FRETURN;

impl Instruction for FRETURN {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let thread = frame.get_thread();
        let _current_frame = thread.borrow_mut().pop_frame();
        let val = frame.get_operand_stack().pop_float();
        thread.borrow_mut().top_frame().borrow_mut().get_operand_stack().push_float(val);

        Ok(())
    }
}

/// Return int from method
#[derive(Default, Debug)]
pub struct IRETURN;

impl Instruction for IRETURN {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let thread = frame.get_thread();
        let _current_frame = thread.borrow_mut().pop_frame();
        let val = frame.get_operand_stack().pop_int();
        thread.borrow_mut().top_frame().borrow_mut().get_operand_stack().push_int(val);

        Ok(())
    }
}

/// Return long from method
#[derive(Default, Debug)]
pub struct LRETURN;

impl Instruction for LRETURN {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let thread = frame.get_thread();
        let _current_frame = thread.borrow_mut().pop_frame();
        let val = frame.get_operand_stack().pop_long();
        thread.borrow_mut().top_frame().borrow_mut().get_operand_stack().push_long(val);

        Ok(())
    }
}
