mod goto;
mod lookupswitch;
mod tableswitch;

#[path = "return.rs"]
mod _return;

pub use self::goto::*;
pub use self::lookupswitch::*;
pub use self::tableswitch::*;
pub use self::_return::*;
