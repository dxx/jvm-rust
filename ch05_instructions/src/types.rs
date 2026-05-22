use std::cell::RefCell;
use std::rc::Rc;
use std::result::Result as StdResult;

/// 通用类型别名

/// 项目内统一的 Result 类型，错误为字符串
pub type Result<T> = StdResult<T, String>;

/// 引用计数 + 内部可变性的容器，便于在多处共享同一可变对象（如常量池、栈帧）
pub type RcRefCell<T> = Rc<RefCell<T>>;

/// 可为空的 RcRefCell：用于局部变量表/操作数栈的引用 slot（null 引用）
pub type OptionalRcRefCell<T> = Option<RcRefCell<T>>;
