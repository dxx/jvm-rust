use super::frame::Frame;
use super::jvm_stack::Stack;
use crate::types::{OptionalRcRefCell, RcRefCell};

/// Java 线程：每个线程有自己的 pc 寄存器和 JVM 栈
pub struct Thread {
    /// 程序计数器（指向当前执行字节码的位置）
    pc: i64,
    /// 线程私有的 JVM 栈
    stack: Stack,
}

impl Thread {
    pub fn new() -> Self {
        Thread {
            pc: 0,
            stack: Stack::new(1024),
        }
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

    pub fn new_frame(&self, _self: RcRefCell<Self>, max_locals: usize, max_stack: usize) -> Frame {
        return Frame::new(_self, max_locals, max_stack);
    }
}
