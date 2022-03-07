use super::{local_vars::LocalVars, operand_stack::OperandStack};
use super::thread::Thread;
use std::rc::Rc;
use std::cell::RefCell;

/// Stack Frame
pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
    next_pc: i64, // The next instruction after the call
    thread: Rc<RefCell<Thread>>,
}

impl Frame {
    pub fn new(thread: Rc<RefCell<Thread>>, max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_size),
            next_pc: 0,
            thread,
        }
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
