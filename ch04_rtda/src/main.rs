#![allow(dead_code)]
#![allow(unused_variables)]

mod cmd;
mod classpath;
mod classfile;
mod rtda;

use crate::cmd::{parse_cmd, Cmd};
use crate::rtda::{Frame, LocalVars, OperandStack};

fn main() {
    let cmd = parse_cmd();

    if cmd.version_flag {
        println!("{}", "version 0.0.1");
    } else if cmd.help_flag || cmd.class == "" {
        cmd.print_usage();
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(cmd: Cmd) {
    let mut frame = Frame::new(100, 100);
    test_local_vars(frame.local_vars_mut());
    test_operand_stack(frame.operand_stack_mut());
}

fn test_local_vars(local_vars: &mut LocalVars) {
    local_vars.set_int(0, 100);
    local_vars.set_int(1, -100);
    local_vars.set_long(2, 2997924580);
    local_vars.set_long(4, -2997924580);
    local_vars.set_float(6, 3.1415926);
    local_vars.set_double(7, 2.71828182845);
    local_vars.set_ref(9, None);

    println!("{}", local_vars.get_int(0));
    println!("{}", local_vars.get_int(1));
    println!("{}", local_vars.get_long(2));
    println!("{}", local_vars.get_long(4));
    println!("{}", local_vars.get_float(6));
    println!("{}", local_vars.get_double(7));
    println!("{:?}", local_vars.get_ref(9));
}

fn test_operand_stack(operand_stack: &mut OperandStack) {
    operand_stack.push_int(100);
    operand_stack.push_int(-100);
    operand_stack.push_long(2997924580);
    operand_stack.push_long(-2997924580);
    operand_stack.push_float(3.1415926);
    operand_stack.push_double(2.71828182845);
    operand_stack.push_ref(None);

    println!("{:?}", operand_stack.pop_ref());
    println!("{}", operand_stack.pop_double());
    println!("{}", operand_stack.pop_float());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_int());
    println!("{}", operand_stack.pop_int());
}
