#![allow(dead_code)]
#![allow(unused_variables)]

mod cmd;
mod classpath;
mod classfile;
mod rtda;
mod instructions;

mod types;

use crate::cmd::{parse_cmd, Cmd};
use crate::classpath::{Classpath, entry::Entry};
use crate::classfile::ClassFile;
use crate::classfile::member_info::MemberInfo;
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

fn start_jvm(cmd: Cmd) {
    let cp = Classpath::parse(&cmd.x_jre_option, &cmd.cp_option);
    let class_name = cmd.class.replace(".", "/");
    let class_file = load_class(class_name, cp);

    match get_main_method(&class_file) {
        Some(member_info) => {
            interpret(member_info);
        },
        None => {
            println!("Main method not found in class {}", &cmd.class);
        }
    }
}

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

fn get_main_method(cf: &ClassFile) -> Option<&MemberInfo> {
    for m in cf.methods() {
        if m.name() == "main" && m.descriptor() == "([Ljava/lang/String;)V" {
            return Some(m)
        }
    }
    None
}
