use crate::types::RcRefCell;
use super::{local_vars::LocalVars, operand_stack::OperandStack};
use super::thread::Thread;
use super::heap::method::Method;

/// Stack Frame
pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
    next_pc: i64, // The next instruction after the call
    thread: RcRefCell<Thread>,
    method: RcRefCell<Method>,
}

impl Frame {
    pub fn new(thread: RcRefCell<Thread>, method: RcRefCell<Method>) -> Self {
        let max_locals = method.borrow().max_locals() as usize;
        let max_stack = method.borrow().max_stack() as usize;
        Frame {
            next_pc: 0,
            thread,
            method,
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack),
        }
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn set_next_pc(&mut self, next_pc: i64) {
        self.next_pc = next_pc;
    }

    pub fn next_pc(&self) -> i64 {
        self.next_pc
    }

    pub fn thread(&self) -> RcRefCell<Thread> {
        self.thread.clone()
    }

    pub fn method(&self) -> RcRefCell<Method> {
        self.method.clone()
    }
}
