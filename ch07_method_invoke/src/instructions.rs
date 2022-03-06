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

use crate::rtda::Thread;
use crate::rtda::method::Method;
use crate::rtda::Frame;
use self::instruction::Instruction;
use self::bytecode_reader::BytecodeReader;
use self::factory::new_instruction;
use std::rc::Rc;
use std::cell::RefCell;

pub fn interpret(method: Rc<RefCell<Method>>) {
    let thread = Rc::new(RefCell::new(Thread::new()));
    let frame = thread.borrow_mut().new_frame(
        thread.clone(),
        method.clone(),
    );
    thread.borrow_mut().push_frame(frame);

    _loop(thread);
}

fn _loop(thread: Rc<RefCell<Thread>>) {
    // let mut frame = thread.borrow_mut().pop_frame();
    let mut reader = BytecodeReader::default();

    loop {
        //let mut frame = thread.borrow_mut().current_frame();
        //let mut f = frame.as_mut().unwrap();
        //let pc = frame.get_next_pc();

        let pc: Option<i64> = {
            Some(thread.borrow().current_frame().get_next_pc())
        };

        thread.borrow_mut().set_pc(pc.unwrap());

        // Decode
        reader.reset(
            thread.borrow().current_frame().get_method().borrow().code(),
            pc.unwrap() as usize
        );

        let opcode = reader.read_u8();
        match new_instruction(opcode) {
            Ok(mut inst) => {
                inst.fetch_operands(&mut reader);
                thread.borrow_mut().current_frame_mut().set_next_pc(reader.pc() as i64);

                // println!("pc: {}, inst:{:?}", thread.borrow_mut().pc(), inst);
                log_instruction(thread.borrow().current_frame(), &inst);

                // Execute
                inst.execute(thread.borrow_mut().current_frame_mut());

                if thread.borrow().is_stack_empty() {
                    break;
                }
            },
            Err(err) => {
                log_frames(thread.clone());
                panic!("{}", err);
            }
        }
    }
}

fn log_instruction(frame: &Frame, inst: &Box<dyn Instruction>) {
    let method = frame.get_method();
    let class_name = method.borrow().get_class().borrow().name();
    let method_name = method.borrow().name();
    let pc = frame.get_thread().borrow().pc();
    println!("{}.{} #{:2} {:?}", class_name, method_name, pc, inst);
}

fn log_frames(thread: Rc<RefCell<Thread>>) {
    while !thread.borrow().is_stack_empty() {
        let frame = thread.borrow_mut().pop_frame();
        let method = frame.as_ref().unwrap().get_method();
        let class_name = method.borrow().get_class().borrow().name();
        println!(">> pc: {:4} {}.{}{}", frame.as_ref().unwrap().get_next_pc(),
            class_name, method.borrow().name(), method.borrow().descriptor());
    }
}
