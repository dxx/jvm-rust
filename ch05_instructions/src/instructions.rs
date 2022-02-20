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
pub use self::comparisons::*;
pub use self::constants::*;
pub use self::control::*;
pub use self::conversions::*;
pub use self::extended::*;
pub use self::loads::*;
pub use self::math::*;
pub use self::stack::*;
pub use self::stores::*;

use crate::classfile::MemberInfo;
use crate::rtda::Thread;
use self::bytecode_reader::BytecodeReader;
use self::factory::new_instruction;

pub fn interpret(method_info: &MemberInfo) {
    match method_info.code_attribute() {
        Some(info) => {
            let mut thread = Thread::new();
            let frame = thread.new_frame(info.max_locals() as usize, info.max_stack() as usize);
            thread.push_frame(frame);

            _loop(thread, info.code());
        }
        None => {}
    }
}

fn _loop(mut thread: Thread, bytecode: Vec<u8>) {
    let mut frame = thread.pop_frame();
    let mut reader = BytecodeReader::default();

    loop {
        let mut f = frame.as_mut().unwrap();
        let pc = f.get_next_pc();
        thread.set_pc(pc);

        // Decode
        reader.reset(bytecode.clone(), pc as usize);
        let opcode = reader.read_u8();
        let mut inst = new_instruction(opcode);

        inst.fetch_operands(&mut reader);
        f.set_next_pc(reader.pc() as i32);

        // Execute
        println!("pc: {}, inst:{:?}", pc, inst);
        inst.execute(&mut f);
    }
}
