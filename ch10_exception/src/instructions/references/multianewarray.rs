#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::OperandStack;
use crate::rtda::class::Class;
use crate::rtda::Object;
use crate::rtda::cp_classref::ClassRef;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use std::rc::Rc;
use std::cell::RefCell;

/// Create new multidimensional array
#[derive(Default, Debug)]
pub struct MULTI_ANEW_ARRAY {
    index: u16,
    dimensions: u8,
}

impl Instruction for MULTI_ANEW_ARRAY {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16();
        self.dimensions = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let method = frame.method();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let arr_class = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<ClassRef>().unwrap().resolved_class(current_class);
        
        let stack = frame.operand_stack_mut();
        let counts = pop_and_check_counts(stack, self.dimensions as usize)?;
        let arr = new_multi_dimensional_array(counts, &arr_class);

        stack.push_ref(Some(Rc::new(RefCell::new(arr))));

        Ok(())
    }
}

fn pop_and_check_counts(stack: &mut OperandStack, dimensions: usize) -> std::result::Result<Vec<i32>, String> {
    let mut counts = vec![0; dimensions];

    for i in (0..dimensions).rev() {
        let val = stack.pop_int();
        counts[i] = val;
        if val < 0 {
            return Err("java.lang.NegativeArraySizeException".into());
        }
    }
    Ok(counts)
}

fn new_multi_dimensional_array(counts: Vec<i32>, arr_class: &Rc<RefCell<Class>>) -> Object {
    let count = counts.get(0).unwrap();
    let mut arr = arr_class.borrow_mut().new_array(arr_class.clone(), *count as usize);

    if counts.len() > 1 {
        let refs = arr.refs_mut();
        for i in 0..refs.len() {
            let obj = new_multi_dimensional_array(
                counts[1..].to_vec(),
            &arr_class.borrow_mut().component_class());
            refs[i] = Some(Rc::new(RefCell::new(obj)));
        }
    }

    arr
}

