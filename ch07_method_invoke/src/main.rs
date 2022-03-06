#![allow(dead_code)]
#![allow(unused_variables)]

mod cmd;
mod classpath;
mod classfile;
mod rtda;
mod instructions;

use crate::cmd::{parse_cmd, Cmd};
use crate::classpath::Classpath;
use crate::rtda::class_loader::ClassLoader;
use crate::instructions::interpret;
use std::rc::Rc;
use std::cell::RefCell;

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
    let class_loader = Rc::new(RefCell::new(ClassLoader::new(cp, cmd.verbose_class_flag)));
    let main_class = class_loader.borrow_mut().load_class(class_loader.clone(), cmd.class.clone());

    let main_method = main_class.borrow_mut().get_main_method();
    match main_method {
        Some(member) => {
            interpret(member.clone(), cmd.verbose_inst_flag);
        },
        None => {
            println!("Main method not found in class {}", &cmd.class);
        }
    }
}
