#![allow(non_camel_case_types)]

use crate::rtda::{Frame, Object};
use super::super::instruction::Instruction;
use std::rc::Rc;
use std::cell::RefCell;

/// Store into reference array
#[derive(Default, Debug)]
pub struct AASTORE;

impl Instruction for AASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let _ref = stack.pop_ref();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_refs = arr_ref.as_mut().unwrap().borrow_mut();
        let refs = b_refs.refs_mut();

        check_index(refs.len(), index);

        refs[index as usize] = _ref;
    }
}

/// Store into byte or boolean array
#[derive(Default, Debug)]
pub struct BASTORE;

impl Instruction for BASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_int();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_bytes = arr_ref.as_mut().unwrap().borrow_mut();
        let bytes = b_bytes.bytes_mut();

        check_index(bytes.len(), index);

        bytes[index as usize] = val as i8;
    }
}

/// Store into char array
#[derive(Default, Debug)]
pub struct CASTORE;

impl Instruction for CASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_int();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_chars = arr_ref.as_mut().unwrap().borrow_mut();
        let chars = b_chars.chars_mut();

        check_index(chars.len(), index);

        chars[index as usize] = val as u16;
    }
}

/// Store into double array
#[derive(Default, Debug)]
pub struct DASTORE;

impl Instruction for DASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_double();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_doubles = arr_ref.as_mut().unwrap().borrow_mut();
        let doubles = b_doubles.doubles_mut();

        check_index(doubles.len(), index);

        doubles[index as usize] = val;
    }
}

/// Store into float array
#[derive(Default, Debug)]
pub struct FASTORE;

impl Instruction for FASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_float();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_floats = arr_ref.as_mut().unwrap().borrow_mut();
        let floats = b_floats.floats_mut();

        check_index(floats.len(), index);

        floats[index as usize] = val;
    }
}

/// Store into int array
#[derive(Default, Debug)]
pub struct IASTORE;

impl Instruction for IASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_int();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_ints = arr_ref.as_mut().unwrap().borrow_mut();
        let ints = b_ints.ints_mut();

        check_index(ints.len(), index);

        ints[index as usize] = val;
    }
}

/// Store into long array
#[derive(Default, Debug)]
pub struct LASTORE;

impl Instruction for LASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_long();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_longs = arr_ref.as_mut().unwrap().borrow_mut();
        let longs = b_longs.longs_mut();

        check_index(longs.len(), index);

        longs[index as usize] = val;
    }
}

/// Store into short array
#[derive(Default, Debug)]
pub struct SASTORE;

impl Instruction for SASTORE {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.get_operand_stack();
        let val = stack.pop_int();
        let index = stack.pop_int();
        let mut arr_ref = stack.pop_ref();

        check_not_none(arr_ref.clone());

        let mut b_shorts = arr_ref.as_mut().unwrap().borrow_mut();
        let shorts = b_shorts.shorts_mut();

        check_index(shorts.len(), index);

        shorts[index as usize] = val as i16;
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
