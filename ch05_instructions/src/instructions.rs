//! 指令集解释器
//!
//! 实现 JVM 字节码指令的解码（Decode）- 取操作数（Fetch）- 执行（Execute）循环。
//! 具体指令按功能分到以下子模块：
//! - constants:  常量入栈（iconst、lconst、fconst、dconst、bipush、sipush、ldc 等）
//! - loads:      从局部变量表加载到操作数栈（iload、lload、fload、dload、aload 等）
//! - stores:     从操作数栈存入局部变量表（istore、lstore 等）
//! - stack:      操作数栈管理（pop、dup、swap）
//! - math:       算术与位运算（add、sub、mul、div、rem、neg、shl、and、or、xor、iinc）
//! - conversions:类型转换（i2l、i2f、f2d、d2i 等）
//! - comparisons:比较指令（lcmp、fcmp、dcmp、if<cond>、if_icmp<cond>、if_acmp<cond>）
//! - control:    控制转移（goto、tableswitch、lookupswitch）
//! - extended:   扩展指令（wide、goto_w、ifnull、ifnonnull）
//! - base:       基础定义（Instruction trait、BytecodeReader、branch 辅助函数）
//! - factory:    根据 opcode 创建对应指令实例的工厂函数

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

use self::bytecode_reader::BytecodeReader;
use self::factory::new_instruction;
use crate::classfile::member_info::MemberInfo;
use crate::rtda::Thread;
use crate::types::RcRefCell;
use std::cell::RefCell;
use std::rc::Rc;

/// 解释执行指定方法的 Code 属性中的字节码
pub fn interpret(method_info: &MemberInfo) {
    match method_info.code_attribute() {
        Some(info) => {
            // 创建主线程
            let thread = Rc::new(RefCell::new(Thread::new()));
            // 为当前方法创建栈帧（局部变量表 + 操作数栈）
            let frame = thread.borrow_mut().new_frame(
                thread.clone(),
                info.max_locals() as usize,
                info.max_stack() as usize,
            );
            thread.borrow_mut().push_frame(frame);

            // 进入取指-译码-执行循环
            _loop(thread, info.code());
        }
        None => {}
    }
}

/// 指令执行主循环：Decode -> Fetch Operands -> Execute
fn _loop(thread: RcRefCell<Thread>, bytecode: Vec<u8>) {
    let frame = thread.borrow_mut().pop_frame().unwrap();
    let mut reader = BytecodeReader::default();

    loop {
        // 1. 获取当前 pc
        let pc = frame.borrow().next_pc();
        thread.borrow_mut().set_pc(pc);

        // 2. Decode：读取 opcode
        reader.reset(bytecode.clone(), pc as usize);
        let opcode = reader.read_u8();
        match new_instruction(opcode) {
            Ok(mut inst) => {
                // 3. Fetch Operands：指令读取自身所需的操作数
                inst.fetch_operands(&mut reader);
                // 更新下一条指令的位置
                frame.borrow_mut().set_next_pc(reader.pc() as i64);

                // 4. Execute：执行指令
                println!("pc: {}, inst:{:?}", pc, inst);
                inst.execute(&mut frame.borrow_mut());
            }
            Err(err) => {
                // 遇到未实现指令时打印当前运行时状态后 panic
                println!("LocalVars: {:?}", frame.borrow_mut().local_vars_mut());
                println!("OperandStack: {:?}", frame.borrow_mut().operand_stack_mut());
                panic!("{}", err);
            }
        }
    }
}
