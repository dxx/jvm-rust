use crate::types::RcRefCell;
use crate::rtda::Frame;
use crate::rtda::ObjectExtra;
use crate::rtda::class::Class;
use crate::rtda::Object;
use crate::rtda::Thread;
use super::registry;
use std::fmt;

const J_THROWABLE: &str = "java/lang/Throwable";

pub fn init() {
    registry(J_THROWABLE.into(), "fillInStackTrace".into(), "(I)Ljava/lang/Throwable;".into(), fill_in_stack_trace);
}

pub struct StackTraceElement {
    file_name: String,
    class_name: String,
    method_name: String,
    line_number: i64,
}

impl fmt::Display for StackTraceElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}({}:{})",
            self.class_name, self.method_name, self.file_name, self.line_number)
    }
}

impl ObjectExtra for Vec<StackTraceElement> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// private native Throwable fillInStackTrace(int dummy);
/// (I)Ljava/lang/Throwable;
fn fill_in_stack_trace(frame: &mut Frame) {
    let mut this = frame.local_vars_mut().get_this();
    frame.operand_stack_mut().push_ref(this.clone());

    let stes = create_stack_trace_elements(this.as_ref().unwrap(), &frame.thread());
    this.as_mut().unwrap().borrow_mut().set_extra(Some(Box::new(stes)));
}

fn create_stack_trace_elements(
    t_obj: &RcRefCell<Object>,
    thread: &RcRefCell<Thread>,
) -> Vec<StackTraceElement> {
    let skip = distance_to_object(t_obj.borrow().class()) + 2;
    let frames = &thread.borrow().get_frames()[(skip as usize)..];
    let mut stes = Vec::new();
    for frame in frames.iter() {
        stes.push(create_stack_trace_element(&frame.borrow()));
    }
    stes
}

fn distance_to_object(class: &RcRefCell<Class>) -> i64 {
    let mut distance = 0;
    let mut c = class.borrow().super_class();
    while let Some(class) = c {
        distance += 1;
        c = class.borrow().super_class();
    }

    distance
}

fn create_stack_trace_element(frame: &Frame) -> StackTraceElement {
    let method = frame.method();
    let class = method.borrow().get_class();
    let file_name = class.borrow().source_file_name();
    let class_name = class.borrow().java_name();
    let method_name = method.borrow().name();
    let line_number = method.borrow().get_line_number(frame.next_pc() - 1);

    StackTraceElement {
        file_name,
        class_name,
        method_name,
        line_number,
    }
}
