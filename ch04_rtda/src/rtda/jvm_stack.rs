use super::frame::Frame;
use crate::types::{OptionalRcRefCell, RcRefCell};
use std::cell::RefCell;
use std::rc::Rc;

/// JVM 栈：每个 Java 线程私有，由若干栈帧组成
/// 这里采用定长数组 + top 指针的简单实现（容量在创建线程时确定）
pub struct Stack {
    /// 栈最大容量（栈帧数）
    max_size: usize,
    /// 栈顶位置（指向下一个空位）
    top: usize,
    /// 栈帧数组；使用 Option<Rc<RefCell<_>>> 以便弹出后仍可被外部持有
    frames: Vec<OptionalRcRefCell<Frame>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Stack {
            max_size,
            top: 0,
            frames: vec![None; max_size],
        }
    }

    /// 入栈一个栈帧；栈满时抛出 StackOverflowError
    pub fn push(&mut self, frame: Frame) {
        if self.top >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }

        self.frames[self.top] = Some(Rc::new(RefCell::new(frame)));
        self.top += 1;
    }

    /// 弹出栈顶帧
    pub fn pop(&mut self) -> OptionalRcRefCell<Frame> {
        if self.top == 0 {
            panic!("jvm stack is empty!");
        }
        self.top -= 1;
        self.frames[self.top].clone()
    }

    /// 获取当前栈顶帧（不弹出）
    pub fn top(&self) -> RcRefCell<Frame> {
        if self.top == 0 {
            panic!("jvm stack is empty!");
        }
        self.frames[self.top - 1].clone().unwrap()
    }
}
