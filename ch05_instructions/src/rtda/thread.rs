use std::rc::Rc;
use std::cell::RefCell;
use super::jvm_stack::Stack;
use super::frame::Frame;

pub struct Thread {
    pc: i32,
    stack: Stack,
}

impl Thread {
    pub fn new() -> Self {
        Thread { pc: 0, stack: Stack::new(1024) }
    }

    pub fn pc(&self) -> i32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Box<Frame>> {
        self.stack.pop()
    }

    pub fn current_frame(&self) -> &Frame {
        self.stack.top()
    }

    pub fn new_frame(&self, _self: Rc<RefCell<Self>>, max_locals: usize, max_stack: usize) -> Frame {
        return Frame::new(_self, max_locals, max_stack);
    }
}