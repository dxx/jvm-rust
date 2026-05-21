//! 运行时数据区（Runtime Data Area）
//!
//! 实现 JVM 规范定义的若干运行时数据结构：
//! - object:         对象（占位实现）
//! - thread:         Java 线程，持有 pc 寄存器和 JVM 栈
//! - jvm_stack:      JVM 栈（由若干栈帧组成）
//! - frame:          栈帧（局部变量表 + 操作数栈）
//! - local_vars:     局部变量表
//! - operand_stack:  操作数栈

mod frame;
mod jvm_stack;
mod local_vars;
mod object;
mod operand_stack;
mod thread;

pub use self::frame::*;
pub use self::local_vars::*;
pub use self::operand_stack::*;
