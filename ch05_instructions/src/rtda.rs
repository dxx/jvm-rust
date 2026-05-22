//! 运行时数据区（Runtime Data Area）
//!
//! 各模块说明：
//! - Frame 增加了 next_pc 和 thread 引用，用于指令跳转和解释器循环
//! - Thread 增加了 new_frame 辅助方法

mod frame;
mod jvm_stack;
mod local_vars;
mod object;
mod operand_stack;
mod thread;

pub use self::frame::*;
pub use self::thread::*;
// pub use self::local_vars::*;
// pub use self::operand_stack::*;
