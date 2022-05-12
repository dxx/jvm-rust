mod base;
mod comparisons;
mod constants;
mod control;
mod conversions;
mod extended;
mod loads;
mod math;
mod stack;
mod stores;
mod references;

mod factory;

pub use self::base::*;

use crate::types::RcRefCell;
use crate::rtda::Thread;
use crate::rtda::method::Method;
use crate::rtda::Frame;
use self::instruction::Instruction;
use self::bytecode_reader::BytecodeReader;
use self::factory::new_instruction;
use std::rc::Rc;
use std::cell::RefCell;

pub fn interpret(method: RcRefCell<Method>, log_inst: bool) {
    let thread = Rc::new(RefCell::new(Thread::new()));
    let frame = thread.borrow_mut().new_frame(
        thread.clone(),
        method.clone(),
    );
    thread.borrow_mut().push_frame(frame);

    _loop(thread, log_inst);
}

fn _loop(thread: RcRefCell<Thread>, log_inst: bool) {
    let mut reader = BytecodeReader::default();

    loop {
        let frame = thread.borrow_mut().current_frame();
        let pc = frame.borrow().next_pc();

        thread.borrow_mut().set_pc(pc);

        // Decode
        reader.reset(
            frame.borrow().method().borrow().code(),
            pc as usize
        );

        let opcode = reader.read_u8();
        match new_instruction(opcode) {
            Ok(mut inst) => {
                inst.fetch_operands(&mut reader);
                frame.borrow_mut().set_next_pc(reader.pc() as i64);

                if log_inst {
                    log_instruction(&frame.borrow(), &inst);
                }

                // Execute
                inst.execute(&mut frame.borrow_mut());
                
                if thread.borrow().is_stack_empty() {
                    break;
                }
            },
            Err(err) => {
                log_frames(&thread);

                panic!("{}", err);
            }
        }
    }
}

fn log_instruction(frame: &Frame, inst: &Box<dyn Instruction>) {
    let method = frame.method();
    let class_name = method.borrow().get_class().borrow().name();
    let method_name = method.borrow().name();
    let pc = frame.thread().borrow().pc();
    println!("{}.{} #{:2} {:?}", class_name, method_name, pc, inst);
}

fn log_frames(thread: &RcRefCell<Thread>) {
    while !thread.borrow().is_stack_empty() {
        let frame = thread.borrow_mut().pop_frame();
        let method = frame.as_ref().unwrap().borrow().method();
        let pc = frame.as_ref().unwrap().borrow().next_pc();
        let class_name = method.borrow().get_class().borrow().name();
        println!(">> pc: {:4} {}.{}{}", pc,
            class_name, method.borrow().name(), method.borrow().descriptor());
    }
}
