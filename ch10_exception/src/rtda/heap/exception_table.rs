use crate::classfile::attribute_info::attr_code::ExceptionTableEntry;
use super::class::Class;
use super::cp_classref::ClassRef;
use super::constant_pool::ConstantPool;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ExceptionTable {
    handlers: Vec<ExceptionHandler>,
}

#[derive(Clone)]
pub struct ExceptionHandler {
    start_pc: i64,
    end_pc: i64,
    handler_pc: i64,
    catch_type: Option<Box<ClassRef>>,
}

impl ExceptionTable {
    pub fn new(entries: &Vec<ExceptionTableEntry>, cp: &Rc<RefCell<ConstantPool>>) -> Self {
        let mut handlers = Vec::new();
        for entry in entries {
            let handler = ExceptionHandler {
                start_pc: entry.start_pc() as i64,
                end_pc: entry.end_pc() as i64,
                handler_pc: entry.handler_pc() as i64,
                catch_type: get_catch_type(entry.catch_type() as usize, cp),
            };
            handlers.push(handler);
        }

        ExceptionTable {
            handlers
        }
    }

    pub fn find_exception_handler(&mut self, ex_class: &Rc<RefCell<Class>>, pc: i64) -> Option<ExceptionHandler> {
        for handler in self.handlers.iter_mut() {
            if pc >= handler.start_pc && pc < handler.end_pc {
                if handler.catch_type.is_none() {
                    return Some(handler.clone()); // Catch all
                }
                let catch_class = handler.catch_type.as_mut().unwrap().resolved_class(ex_class.clone());
                if catch_class.eq(ex_class) || ex_class.borrow().is_sub_class_of(&catch_class) {
                    return Some(handler.clone()); 
                }
            }
        }

        None
    }
}

impl ExceptionHandler {
    pub fn handler_pc(&self) -> i64 {
        self.handler_pc
    }
}

fn get_catch_type(index: usize, cp: &Rc<RefCell<ConstantPool>>) -> Option<Box<ClassRef>> {
    if index == 0 {
        return None;
    }
    let b_cp = cp.borrow();
    let class_ref = b_cp.get_constant(index).as_any().downcast_ref::<ClassRef>().unwrap();

    Some(Box::new(class_ref.clone()))
}
