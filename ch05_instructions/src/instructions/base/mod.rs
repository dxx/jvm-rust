//! 基础定义模块，包含字节码读取器、指令 trait 和分支跳转辅助函数

pub mod bytecode_reader;
pub mod instruction;

use super::super::rtda::Frame;

/// 分支跳转辅助函数：根据偏移量修改线程的下一条 PC
pub fn branch(frame: &mut Frame, offset: i64) {
    let pc = frame.thread().borrow().pc();
    let next_pc = pc + offset;
    frame.set_next_pc(next_pc);
}
