use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use super::jvm_stack::Stack;
use super::frame::Frame;
use super::heap::method::Method;

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

    pub fn pop_frame(&mut self) -> OptionalRcRefCell<Frame> {
        self.stack.pop()
    }

    pub fn current_frame(&self) -> RcRefCell<Frame> {
        self.stack.top()
    }

    pub fn top_frame(&mut self) -> RcRefCell<Frame> {
        self.stack.top()
    }

    pub fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn new_frame(&self, _self: RcRefCell<Self>, method: RcRefCell<Method>) -> Frame {
        return Frame::new(_self, method);
    }
}