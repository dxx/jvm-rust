use super::jvm_stack::Stack;
use super::frame::Frame;
use super::heap::method::Method;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Thread {
    pc: i64,
    stack: Stack,
}

impl Thread {
    pub fn new() -> Self {
        Thread { pc: 0, stack: Stack::new(1024) }
    }

    pub fn pc(&self) -> i64 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: i64) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Rc<RefCell<Frame>>> {
        self.stack.pop()
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        self.stack.top()
    }

    pub fn new_frame(&self, _self: Rc<RefCell<Self>>, method: Rc<RefCell<Method>>) -> Frame {
        return Frame::new(_self, method);
    }
}