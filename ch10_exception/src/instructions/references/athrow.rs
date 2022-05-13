use crate::types::RcRefCell;
use crate::rtda::Frame;
use crate::rtda::Thread;
use crate::rtda::Object;
use crate::rtda::string_pool::rust_string;
use crate::native;
use super::super::instruction::Instruction;

/// Throw exception or error
#[derive(Default, Debug)]
pub struct ATHROW;

impl Instruction for ATHROW {
    fn execute(&mut self, frame: &mut Frame) -> crate::Result<()> {
        let ex = frame.operand_stack_mut().pop_ref();
        if ex.is_none() {
            return Err("java.lang.NullPointerException".into());
        }

        let thread = frame.thread();
        if !find_and_goto_exception_handler(&thread, ex.clone().unwrap()) {
            thread.borrow_mut().clear_stack();
        
            handle_uncaught_exception(ex.unwrap());
        }

        Ok(())
    }
}

fn find_and_goto_exception_handler(
    thread: &RcRefCell<Thread>,
    ex: RcRefCell<Object>,
) -> bool {
    loop {
        let frame = thread.borrow().current_frame();
        let frame_mut = unsafe { frame.as_ptr().as_mut().unwrap() };
        let pc = frame_mut.next_pc() - 1;

        let handler_pc = frame_mut.method().borrow_mut()
            .find_exception_handler(ex.borrow().class(), pc);
        if handler_pc > 0 {
            let stack = frame_mut.operand_stack_mut();
            stack.clear();
            stack.push_ref(Some(ex));
            frame_mut.set_next_pc(handler_pc);
            return true;
        }

        thread.borrow_mut().pop_frame();
        if thread.borrow().is_stack_empty() {
            break;
        }
    }

    false
}

fn handle_uncaught_exception(ex: RcRefCell<Object>) {
    let mut ex_mut = ex.borrow_mut();
    let class_name = ex_mut.class().borrow().java_name();

    let j_msg = ex_mut.get_ref_var(
        "detailMessage".into(), "Ljava/lang/String;".into());
    let r_msg = rust_string(&j_msg);

    println!("{}: {}", class_name, r_msg);

    let stes = ex_mut.extra().unwrap().as_any()
        .downcast_ref::<Vec<native::throwable::StackTraceElement>>().unwrap();
    for ste in stes.iter() {
        println!("\tat {}", ste);
    }
}
