#![allow(dead_code)]
#![allow(unused_variables)]

mod classfile;
mod classpath;
mod cmd;
mod rtda;

mod types;

use crate::cmd::{parse_cmd, Cmd};
use crate::rtda::{Frame, LocalVars, OperandStack};

fn main() {
    // 解析命令行参数
    let cmd = parse_cmd();

    if cmd.version_flag {
        // -version：打印版本号
        println!("{}", "version 0.0.1");
    } else if cmd.help_flag || cmd.class == "" {
        // -help 或未指定主类时，打印使用说明
        cmd.print_usage();
    } else {
        // 启动 JVM
        start_jvm(cmd);
    }
}

/// 本章演示运行时数据区：构造一个栈帧，分别测试局部变量表和操作数栈
fn start_jvm(cmd: Cmd) {
    // max_locals = 100, max_stack = 100
    let mut frame = Frame::new(100, 100);
    test_local_vars(frame.local_vars_mut());
    test_operand_stack(frame.operand_stack_mut());
}

/// 测试局部变量表：分别写入并读取 int/long/float/double/ref 各种类型
fn test_local_vars(local_vars: &mut LocalVars) {
    local_vars.set_int(0, 100);
    local_vars.set_int(1, -100);
    // long 占 2 个 slot（2、3）
    local_vars.set_long(2, 2997924580);
    // long 占 2 个 slot（4、5）
    local_vars.set_long(4, -2997924580);
    local_vars.set_float(6, 3.1415926);
    // double 占 2 个 slot（7、8）
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

/// 测试操作数栈：依次入栈不同类型，再按栈的后进先出顺序弹出
fn test_operand_stack(operand_stack: &mut OperandStack) {
    operand_stack.push_int(100);
    operand_stack.push_int(-100);
    operand_stack.push_long(2997924580);
    operand_stack.push_long(-2997924580);
    operand_stack.push_float(3.1415926);
    operand_stack.push_double(2.71828182845);
    operand_stack.push_ref(None);

    // 按入栈相反顺序弹出
    println!("{:?}", operand_stack.pop_ref());
    println!("{}", operand_stack.pop_double());
    println!("{}", operand_stack.pop_float());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_int());
    println!("{}", operand_stack.pop_int());
}
