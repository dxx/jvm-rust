use super::thread::Thread;
use super::{local_vars::LocalVars, operand_stack::OperandStack};
use crate::types::RcRefCell;

/// 栈帧（Stack Frame）：每个方法调用对应一个栈帧
/// 包含该方法的局部变量表、操作数栈，以及用于指令跳转的 next_pc
pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
    /// 下一条指令的 pc（用于分支/跳转指令修改执行流）
    next_pc: i64,
    /// 所属线程的引用（用于 branch 等指令读取当前 pc）
    thread: RcRefCell<Thread>,
}

impl Frame {
    pub fn new(thread: RcRefCell<Thread>, max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_size),
            next_pc: 0,
            thread,
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

    pub fn thread(&self) -> &RcRefCell<Thread> {
        &self.thread
    }
}
