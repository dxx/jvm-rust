use super::{local_vars::LocalVars, operand_stack::OperandStack};
use super::thread::Thread;
use super::heap::method::Method;
use std::rc::Rc;
use std::cell::RefCell;

/// Stack Frame
pub struct Frame {
    pub lower: Option<Box<Frame>>, // Stack is implemented as linked list
    local_vars: LocalVars,
    operand_stack: OperandStack,
    next_pc: i64, // The next instruction after the call
    thread: Rc<RefCell<Thread>>,
    method: Rc<RefCell<Method>>,
}

impl Frame {
    pub fn new(thread: Rc<RefCell<Thread>>, method: Rc<RefCell<Method>>) -> Self {
        let max_locals = method.borrow().max_locals() as usize;
        let max_stack = method.borrow().max_stack() as usize;
        Frame {
            lower: None,
            next_pc: 0,
            thread,
            method,
            local_vars: LocalVars::new(max_stack),
            operand_stack: OperandStack::new(max_stack),
        }
    }

    pub fn set_lower(&mut self, lower: Option<Box<Frame>>) {
        self.lower = lower;
    }

    pub fn get_local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn get_operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn set_next_pc(&mut self, next_pc: i64) {
        self.next_pc = next_pc;
    }

    pub fn get_next_pc(&self) -> i64 {
        self.next_pc
    }

    pub fn get_thread(&self) -> &Rc<RefCell<Thread>> {
        &self.thread
    }
}
