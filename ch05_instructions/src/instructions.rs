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

mod factory;

pub use self::base::*;

use crate::types::RcRefCell;
use crate::classfile::member_info::MemberInfo;
use crate::rtda::Thread;
use std::rc::Rc;
use std::cell::RefCell;
use self::bytecode_reader::BytecodeReader;
use self::factory::new_instruction;

pub fn interpret(method_info: &MemberInfo) {
    match method_info.code_attribute() {
        Some(info) => {
            let thread = Rc::new(RefCell::new(Thread::new()));
            let frame = thread.borrow_mut().new_frame(
                thread.clone(),
                info.max_locals() as usize,
                info.max_stack() as usize
            );
            thread.borrow_mut().push_frame(frame);

            _loop(thread, info.code());
        }
        None => {}
    }
}

fn _loop(thread: RcRefCell<Thread>, bytecode: Vec<u8>) {
    let frame = thread.borrow_mut().pop_frame().unwrap();
    let mut reader = BytecodeReader::default();

    loop {
        let pc = frame.borrow().next_pc();
        thread.borrow_mut().set_pc(pc);

        // Decode
        reader.reset(bytecode.clone(), pc as usize);
        let opcode = reader.read_u8();
        match new_instruction(opcode) {
            Ok(mut inst) => {
                inst.fetch_operands(&mut reader);
                frame.borrow_mut().set_next_pc(reader.pc() as i64);

                // Execute
                println!("pc: {}, inst:{:?}", pc, inst);
                inst.execute(&mut frame.borrow_mut());
            },
            Err(err) => {
                println!("LocalVars: {:?}", frame.borrow_mut().local_vars_mut());
                println!("OperandStack: {:?}", frame.borrow_mut().operand_stack_mut());
                panic!("{}", err);
            }
        }
    }
}
