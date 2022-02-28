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
    // let class_file = load_class(cmd.class.clone(), cp);
    let class_loader = Rc::new(RefCell::new(ClassLoader::new(cp)));
    let main_class = class_loader.borrow_mut().load_class(&class_loader, cmd.class.clone());
    let main_class = main_class.borrow();
    match main_class.get_main_method() {
        Some(member) => {
            interpret(member);
        },
        None => {
            println!("Main method not found in class {}", &cmd.class);
        }
    }
}

// fn load_class(class_name: String, class_path: Classpath) -> ClassFile {
//     let class_data = match class_path.read_class(&class_name) {
//         Ok(class_data) => class_data,
//         Err(err) => panic!("{}", err),
//     };

//     let class_file = match ClassFile::parse(class_data) {
//         Ok(class_file) => class_file,
//         Err(err) => panic!("{}", err),
//     };

//     class_file
// }

// fn get_main_method(cf: &ClassFile) -> Option<&MemberInfo> {
//     for m in cf.methods() {
//         if m.name() == "main" && m.descriptor() == "([Ljava/lang/String;)V" {
//             return Some(m)
//         }
//     }
//     None
// }
