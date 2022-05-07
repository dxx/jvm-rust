use crate::rtda::Frame;
use crate::rtda::string_pool;
use super::registry;

const J_STRING: &str = "java/lang/String";

pub fn init() {
    registry(J_STRING.into(), "intern".into(), "()Ljava/lang/String;".into(), intern);
}

// public native String intern();
// ()Ljava/lang/String;
fn intern(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    let interned = string_pool::intern_string(this.as_ref().unwrap());

    frame.operand_stack_mut().push_ref(Some(interned));
}
