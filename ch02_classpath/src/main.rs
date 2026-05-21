mod classpath;
mod cmd;

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

/// 启动 JVM：构造 Classpath，并尝试读取主类的字节码
fn start_jvm(cmd: Cmd) {
    // 根据 -Xjre 和 -cp/-classpath 构造完整的 classpath（boot/ext/user 三部分）
    let mut cp = Classpath::parse(&cmd.x_jre_option, &cmd.cp_option);

    println!(
        "classpath: {} class: {} args: {:?}",
        cp, cmd.class, cmd.args
    );

    // 类名中的 '.' 转换为路径分隔符 '/'，例如 java.lang.Object => java/lang/Object
    let class_name = cmd.class.replace(".", "/");
    // 从 classpath 查找并读取 class 文件内容（字节数组）
    let class_data = match cp.read_class(&class_name) {
        Ok(class_data) => class_data,
        Err(err) => {
            panic!("Could not find or load main class {}: {}", cmd.class, err);
        }
    };
    println!("class data: {:?}", class_data);
}
