#![allow(non_camel_case_types)]

use crate::rtda::{Frame, Object};
use super::super::instruction::Instruction;
use std::rc::Rc;
use std::cell::RefCell;

/// Load reference from array
#[derive(Default, Debug)]
pub struct AALOAD;

impl Instruction for AALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;
        
        let mut b_refs = arr_ref.as_mut().unwrap().borrow_mut();
        let refs = b_refs.refs_mut();

        check_index(refs.len(), index)?;

        stack.push_ref(refs.get(index as usize).unwrap().clone());

        Ok(())
    }
}

/// Load byte or boolean from array
#[derive(Default, Debug)]
pub struct BALOAD;

impl Instruction for BALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_bytes = arr_ref.as_mut().unwrap().borrow_mut();
        let bytes = b_bytes.bytes_mut();

        check_index(bytes.len(), index)?;

        stack.push_int(*bytes.get(index as usize).unwrap() as i32);

        Ok(())
    }
}

/// Load char from array
#[derive(Default, Debug)]
pub struct CALOAD;

impl Instruction for CALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_chars = arr_ref.as_mut().unwrap().borrow_mut();
        let chars = b_chars.chars_mut();

        check_index(chars.len(), index)?;

        stack.push_int(*chars.get(index as usize).unwrap() as i32);

        Ok(())
    }
}

/// Load double from array
#[derive(Default, Debug)]
pub struct DALOAD;

impl Instruction for DALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_doubles = arr_ref.as_mut().unwrap().borrow_mut();
        let doubles = b_doubles.doubles_mut();

        check_index(doubles.len(), index)?;

        stack.push_double(*doubles.get(index as usize).unwrap());

        Ok(())
    }
}

/// Load float from array
#[derive(Default, Debug)]
pub struct FALOAD;

impl Instruction for FALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_floats = arr_ref.as_mut().unwrap().borrow_mut();
        let floats = b_floats.floats_mut();

        check_index(floats.len(), index)?;

        stack.push_float(*floats.get(index as usize).unwrap());

        Ok(())
    }
}

/// Load int from array
#[derive(Default, Debug)]
pub struct IALOAD;

impl Instruction for IALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_ints = arr_ref.as_mut().unwrap().borrow_mut();
        let ints = b_ints.ints_mut();

        check_index(ints.len(), index)?;

        stack.push_int(*ints.get(index as usize).unwrap());

        Ok(())
    }
}

/// Load long from array
#[derive(Default, Debug)]
pub struct LALOAD;

impl Instruction for LALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_longs = arr_ref.as_mut().unwrap().borrow_mut();
        let longs = b_longs.longs_mut();

        check_index(longs.len(), index)?;

        stack.push_long(*longs.get(index as usize).unwrap());

        Ok(())
    }
}

/// Load short from array
#[derive(Default, Debug)]
pub struct SALOAD;

impl Instruction for SALOAD {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let stack = frame.operand_stack_mut();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone())?;

        let mut b_shorts = arr_ref.as_mut().unwrap().borrow_mut();
        let shorts = b_shorts.shorts_mut();

        check_index(shorts.len(), index)?;

        stack.push_int(*shorts.get(index as usize).unwrap() as i32);

        Ok(())
    }
}

fn check_not_none(_ref: Option<Rc<RefCell<Object>>>) -> crate::Result<()> {
    if _ref.is_none() {
        return Err("java.lang.NullPointerException".into());
    }
    Ok(())
}

fn check_index(arr_len: usize, index: i32) -> crate::Result<()> {
    if index < 0 || index >= arr_len as i32 {
        return Err("java.lang.ArrayIndexOutOfBoundsException".into());
    }
    Ok(())
}
