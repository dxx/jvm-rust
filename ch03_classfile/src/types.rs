use std::cell::RefCell;
use std::rc::Rc;
use std::result::Result as StdResult;

/// 通用类型别名

/// 项目内统一的 Result 类型，错误为字符串
pub type Result<T> = StdResult<T, String>;

/// 引用计数 + 内部可变性的容器，便于在 class 解析期间多处共享常量池
pub type RcRefCell<T> = Rc<RefCell<T>>;
