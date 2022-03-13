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
mod reserved;

mod factory;

pub use self::base::*;

use crate::rtda::Thread;
use crate::rtda::Frame;
use crate::rtda::Object;
use crate::rtda::method::Method;
use self::instruction::Instruction;
use self::bytecode_reader::BytecodeReader;
use self::factory::new_instruction;
use std::rc::Rc;
use std::cell::RefCell;

pub fn interpret(method: Rc<RefCell<Method>>, log_inst: bool, args: Vec<String>) {
    let thread = Rc::new(RefCell::new(Thread::new()));
    let mut frame = thread.borrow_mut().new_frame(
        thread.clone(),
        method.clone(),
    );
    
    let j_args = create_args_array(method, args);
    frame.get_local_vars().set_ref(0, Some(j_args));

    thread.borrow_mut().push_frame(frame);

    _loop(thread, log_inst);
}

fn create_args_array(
    method: Rc<RefCell<Method>>,
    args: Vec<String>,
) -> Rc<RefCell<Object>> {
    let class = method.borrow().get_class();
    let loader = class.borrow_mut().loader().unwrap();
    let constant_pool = class.borrow_mut().constant_pool();
    let mut constant_pool = constant_pool.borrow_mut();
    let string_pool = constant_pool.string_pool_mut();

    let string_class = loader.borrow_mut().load_class(
        loader.clone(),
    "java/lang/String".into());
    let array_class = string_class.borrow_mut().array_class();
    let mut args_arr = array_class.borrow_mut().new_array(array_class.clone(), args.len());
    
    let j_args = args_arr.refs_mut();
    for i in 0..args.len() {
        let j_str = string_pool.jstring(loader.clone(), args[i].clone());
        j_args[i] = Some(j_str);
    }

    Rc::new(RefCell::new(args_arr))
}

fn _loop(thread: Rc<RefCell<Thread>>, log_inst: bool) {
    let mut reader = BytecodeReader::default();

    loop {
        let frame = thread.borrow_mut().current_frame();
        let pc = frame.borrow().get_next_pc();

        thread.borrow_mut().set_pc(pc);

        // Decode
        reader.reset(
            frame.borrow().get_method().borrow().code(),
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
                let result = inst.execute(&mut frame.borrow_mut());
                if result.is_err() {
                    log_frames(&thread);
        
                    panic!("{}", result.err().unwrap());
                }
                
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
    let method = frame.get_method();
    let class_name = method.borrow().get_class().borrow().name();
    let method_name = method.borrow().name();
    let pc = frame.get_thread().borrow().pc();
    println!("{}.{} #{:2} {:?}", class_name, method_name, pc, inst);
}

fn log_frames(thread: &Rc<RefCell<Thread>>) {
    while !thread.borrow().is_stack_empty() {
        let frame = thread.borrow_mut().pop_frame();
        let method = frame.as_ref().unwrap().borrow().get_method();
        let pc = frame.as_ref().unwrap().borrow().get_next_pc();
        let class_name = method.borrow().get_class().borrow().name();
        println!(">> pc: {:4} {}.{}{}", pc,
            class_name, method.borrow().name(), method.borrow().descriptor());
    }
}
