//! Instruction trait：所有 JVM 指令的公共接口

use std::fmt::Debug;

use super::bytecode_reader::BytecodeReader;
use crate::rtda::Frame;

/// 指令 trait，所有具体指令类型都需要实现该接口
pub trait Instruction: Debug {
    /// 从字节码中读取操作数（默认空实现，适用于无操作数指令）
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // Nothing to do
    }

    /// 执行指令逻辑，修改当前栈帧状态
    fn execute(&mut self, frame: &mut Frame);
}
