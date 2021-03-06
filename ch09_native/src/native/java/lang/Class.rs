use crate::rtda::Frame;
use crate::rtda::ClassData;
use crate::rtda::string_pool;
use super::registry;

const J_CLASS: &str = "java/lang/Class";

pub fn init() {
    registry(J_CLASS.into(), "getPrimitiveClass".into(), "(Ljava/lang/String;)Ljava/lang/Class;".into(), get_primitive_class);
    registry(J_CLASS.into(), "getName0".into(), "()Ljava/lang/String;".into(), get_name0);
    registry(J_CLASS.into(), "desiredAssertionStatus0".into(), "(Ljava/lang/Class;)Z".into(), desired_assertion_status0);
}

/// static native Class<?> getPrimitiveClass(String name);
/// (Ljava/lang/String;)Ljava/lang/Class;
fn get_primitive_class(frame: &mut Frame) {
    let name_obj = frame.local_vars_mut().get_ref(0);
    let name = string_pool::rust_string(name_obj.as_ref().unwrap());

    let current_class = frame.method().borrow().get_class();
    let loader = current_class.borrow_mut().loader().unwrap();
    let class = loader.borrow_mut().load_class(loader.clone(), name).borrow().j_class();

    frame.operand_stack_mut().push_ref(class);
}

/// private native String getName0();
/// ()Ljava/lang/String;
fn get_name0(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    
    let current_class = frame.method().borrow().get_class();
    let loader = current_class.borrow_mut().loader().unwrap();
    let string_pool = current_class.borrow_mut().string_pool();

    let name = this.unwrap().borrow().extra().unwrap()
        .as_any().downcast_ref::<ClassData>().unwrap().java_name();
    let name_obj = string_pool.borrow_mut().jstring(loader.clone(), name);

    frame.operand_stack_mut().push_ref(Some(name_obj));
}

/// private static native boolean desiredAssertionStatus0(Class<?> clazz);
/// (Ljava/lang/Class;)Z
fn desired_assertion_status0(frame: &mut Frame) {
    frame.operand_stack_mut().push_boolean(false);
}
