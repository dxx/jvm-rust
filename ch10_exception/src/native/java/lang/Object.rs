use crate::rtda::Frame;
use super::registry;

const J_OBJECT: &str = "java/lang/Object";

pub fn init() {
    registry(J_OBJECT.into(), "getClass".into(), "()Ljava/lang/Class;".into(), get_class);
    registry(J_OBJECT.into(), "hashCode".into(), "()I".into(), hash_code);
}

/// public final native Class<?> getClass();
/// ()Ljava/lang/Class;
fn get_class(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    let class = this.unwrap().borrow().class().borrow().j_class();
    frame.operand_stack_mut().push_ref(class);
}

/// public native int hashCode();
/// ()I
fn hash_code(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    let hash = this.unwrap().as_ptr() as i32;
    frame.operand_stack_mut().push_int(hash);
}
