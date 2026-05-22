//! 常量指令子模块：将常量压入操作数栈，以及空操作指令

mod consts;
mod ipush;
mod nop;

pub use self::consts::*;
pub use self::ipush::*;
pub use self::nop::*;
