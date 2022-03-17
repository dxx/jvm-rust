#![allow(dead_code)]
#![allow(unused_variables)]

mod cmd;
mod classpath;
mod classfile;

use crate::cmd::{parse_cmd, Cmd};
use crate::classpath::{Classpath, entry::Entry};
use crate::classfile::ClassFile;

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
    
    print_class_info(class_file);
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

fn print_class_info(class_file: ClassFile) {
    println!("version: {}.{}", class_file.major_version(), class_file.minor_version());
    println!("constants count: {}", class_file.constant_pool().borrow().infos.len());
    println!("access flags: 0x{:x}", class_file.access_flags());
    println!("this class: {}", class_file.class_name());
    println!("super class: {}", class_file.super_class_name());
    println!("interfaces: {:?}", class_file.interface_names());
    println!("fields count: {:?}", class_file.fields().len());
    for field in class_file.fields() {
        println!(" {}", field.name());
    }
    println!("methods count: {:?}", class_file.methods().len());
    for method in class_file.methods() {
        println!(" {}", method.name());
    }
}
