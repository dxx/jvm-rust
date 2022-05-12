use std::result::Result as StdResult;
use std::rc::Rc;
use std::cell::RefCell;

/// Type alias

pub type Result<T> = StdResult<T, String>;

pub type RcRefCell<T> = Rc<RefCell<T>>;

pub type OptionalRcRefCell<T> = Option<Rc<RefCell<T>>>;
