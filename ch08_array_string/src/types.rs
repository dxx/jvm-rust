use std::result::Result as StdResult;

/// Type alias

pub type Result<T> = StdResult<T, String>;
