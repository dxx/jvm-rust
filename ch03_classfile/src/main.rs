#![allow(dead_code)]
#![allow(unused_variables)]

mod classfile;
mod classpath;
mod cmd;

mod types;

use crate::classfile::ClassFile;
use crate::classpath::{entry::Entry, Classpath};
use crate::cmd::{parse_cmd, Cmd};

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

/// 启动 JVM：构造 classpath，加载主类，并打印其结构化信息
fn start_jvm(cmd: Cmd) {
    let cp = Classpath::parse(&cmd.x_jre_option, &cmd.cp_option);
    // 类名中的 '.' 转换为路径分隔符 '/'
    let class_name = cmd.class.replace(".", "/");
    let class_file = load_class(class_name, cp);

    print_class_info(class_file);
}

/// 通过 classpath 读取类字节数据，并解析为 ClassFile 结构
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

/// 打印 class 文件的关键信息：版本号、常量池数量、访问标志、类名、超类名、接口、字段、方法
fn print_class_info(class_file: ClassFile) {
    println!(
        "version: {}.{}",
        class_file.major_version(),
        class_file.minor_version()
    );
    println!(
        "constants count: {}",
        class_file.constant_pool().borrow().infos.len()
    );
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
