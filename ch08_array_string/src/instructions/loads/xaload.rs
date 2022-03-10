#![allow(non_camel_case_types)]

use crate::rtda::{Frame, Object};
use super::super::instruction::Instruction;
use std::rc::Rc;
use std::cell::RefCell;

/// Load reference from array
#[derive(Default, Debug)]
pub struct AALOAD;

impl Instruction for AALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());
        
        let mut b_refs = arr_ref.as_mut().unwrap().borrow_mut();
        let refs = b_refs.refs_mut();

        check_index(refs.len(), index);

        stack.push_ref(refs.get(index as usize).unwrap().clone());
    }
}

/// Load byte or boolean from array
#[derive(Default, Debug)]
pub struct BALOAD;

impl Instruction for BALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_bytes = arr_ref.as_mut().unwrap().borrow_mut();
        let bytes = b_bytes.bytes_mut();

        check_index(bytes.len(), index);

        stack.push_int(*bytes.get(index as usize).unwrap() as i32);
    }
}

/// Load char from array
#[derive(Default, Debug)]
pub struct CALOAD;

impl Instruction for CALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_chars = arr_ref.as_mut().unwrap().borrow_mut();
        let chars = b_chars.chars_mut();

        check_index(chars.len(), index);

        stack.push_int(*chars.get(index as usize).unwrap() as i32);
    }
}

/// Load double from array
#[derive(Default, Debug)]
pub struct DALOAD;

impl Instruction for DALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_doubles = arr_ref.as_mut().unwrap().borrow_mut();
        let doubles = b_doubles.doubles_mut();

        check_index(doubles.len(), index);

        stack.push_double(*doubles.get(index as usize).unwrap());
    }
}

/// Load float from array
#[derive(Default, Debug)]
pub struct FALOAD;

impl Instruction for FALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_floats = arr_ref.as_mut().unwrap().borrow_mut();
        let floats = b_floats.floats_mut();

        check_index(floats.len(), index);

        stack.push_float(*floats.get(index as usize).unwrap());
    }
}

/// Load int from array
#[derive(Default, Debug)]
pub struct IALOAD;

impl Instruction for IALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_ints = arr_ref.as_mut().unwrap().borrow_mut();
        let ints = b_ints.ints_mut();

        check_index(ints.len(), index);

        stack.push_int(*ints.get(index as usize).unwrap());
    }
}

/// Load long from array
#[derive(Default, Debug)]
pub struct LALOAD;

impl Instruction for LALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_longs = arr_ref.as_mut().unwrap().borrow_mut();
        let longs = b_longs.longs_mut();

        check_index(longs.len(), index);

        stack.push_long(*longs.get(index as usize).unwrap());
    }
}

/// Load short from array
#[derive(Default, Debug)]
pub struct SALOAD;

impl Instruction for SALOAD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_shorts = arr_ref.as_mut().unwrap().borrow_mut();
        let shorts = b_shorts.shorts_mut();

        check_index(shorts.len(), index);

        stack.push_int(*shorts.get(index as usize).unwrap() as i32);
    }
}

fn check_not_none(_ref: Option<Rc<RefCell<Object>>>) {
    if _ref.is_none() {
        panic!("java.lang.NullPointerException");
    }
}

fn check_index(arr_len: usize, index: i32) {
    if index < 0 || index >= arr_len as i32 {
        panic!("java.lang.ArrayIndexOutOfBoundsException")
    }
}
