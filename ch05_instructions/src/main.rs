//! ch05: 指令集和解释器
//!
//! 本章在 ch04 运行时数据区的基础上，实现 JVM 字节码指令的解码与执行。
//! 主要新增模块：
//! - instructions: 各类字节码指令的实现
//! - rtda: 运行时数据区（Frame 增加了 next_pc 和 thread 引用）
//! - classfile/classpath/cmd/types: 类文件解析、类路径、命令行参数等支持模块

#![allow(dead_code)]
#![allow(unused_variables)]

mod classfile;
mod classpath;
mod cmd;
mod instructions;
mod rtda;

mod types;

use crate::classfile::member_info::MemberInfo;
use crate::classfile::ClassFile;
use crate::classpath::{entry::Entry, Classpath};
use crate::cmd::{parse_cmd, Cmd};
use crate::instructions::interpret;

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

/// 启动 JVM：加载指定类，找到 main 方法并开始解释执行
fn start_jvm(cmd: Cmd) {
    let cp = Classpath::parse(&cmd.x_jre_option, &cmd.cp_option);
    let class_name = cmd.class.replace(".", "/");
    let class_file = load_class(class_name, cp);

    match get_main_method(&class_file) {
        Some(member_info) => {
            interpret(member_info);
        }
        None => {
            println!("Main method not found in class {}", &cmd.class);
        }
    }
}

/// 根据类名从类路径中读取并解析 .class 文件
fn load_class(class_name: String, mut class_path: Classpath) -> ClassFile {
    let class_data = match class_path.read_class(&class_name) {
        Ok(class_data) => class_data,
        Err(err) => panic!("{}", err),
    };

    let class_file = match ClassFile::parse(class_data) {
        Ok(class_file) => class_file,
        Err(err) => panic!("{}", err),
    };

    class_file
}

/// 在 ClassFile 的方法表中查找 main 方法
/// 签名: public static void main(String[] args)
fn get_main_method(cf: &ClassFile) -> Option<&MemberInfo> {
    for m in cf.methods() {
        if m.name() == "main" && m.descriptor() == "([Ljava/lang/String;)V" {
            return Some(m);
        }
    }
    None
}
