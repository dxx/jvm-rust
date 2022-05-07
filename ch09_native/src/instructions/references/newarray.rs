#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::class::Class;
use crate::rtda::class_loader::ClassLoader;
use super::super::instruction::Instruction;
use super::super::instruction::Result;
use super::super::bytecode_reader::BytecodeReader;
use std::rc::Rc;
use std::cell::RefCell;

const AT_BOOLEAN: u8 =  4;
const AT_CHAR: u8    =  5;
const AT_FLOAT: u8   =  6;
const AT_DOUBLE: u8  =  7;
const AT_BYTE: u8    =  8;
const AT_SHORT: u8   =  9;
const AT_INT: u8     = 10;
const AT_LONG: u8    = 11;

/// Create new array
#[derive(Default, Debug)]
pub struct NEW_ARRAY {
    atype: u8,
}

impl Instruction for NEW_ARRAY {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.atype = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let method = frame.method();
        let stack = frame.operand_stack_mut();
        let count = stack.pop_int();
        if count < 0 {
            return Err("java.lang.NegativeArraySizeException".into());
        }

        let class_loader = method.borrow().get_class().borrow().loader();
        let arr_class = get_primitive_array_class(class_loader.unwrap(), self.atype)?;
        let arr = arr_class.borrow_mut().new_array(arr_class.clone(), count as usize);
        stack.push_ref(Some(Rc::new(RefCell::new(arr))));

        Ok(())
    }
}

fn get_primitive_array_class(loader: Rc<RefCell<ClassLoader>>, atype: u8) -> std::result::Result<Rc<RefCell<Class>>, String> {
    let class = match atype {
        AT_BOOLEAN => {
            loader.borrow_mut().load_class(loader.clone(), "[Z".into())
        },
        AT_BYTE => {
            loader.borrow_mut().load_class(loader.clone(), "[B".into())
        },
        AT_CHAR => {
            loader.borrow_mut().load_class(loader.clone(), "[C".into())
        },
        AT_SHORT => {
            loader.borrow_mut().load_class(loader.clone(), "[S".into())
        },
        AT_INT => {
            loader.borrow_mut().load_class(loader.clone(), "[I".into())
        },
        AT_LONG => {
            loader.borrow_mut().load_class(loader.clone(), "[L".into())
        },
        AT_FLOAT => {
            loader.borrow_mut().load_class(loader.clone(), "[F".into())
        },
        AT_DOUBLE => {
            loader.borrow_mut().load_class(loader.clone(), "[D".into())
        },
        _ => {
            return Err("Invalid atype!".into());
        }
    };
    Ok(class)
}
