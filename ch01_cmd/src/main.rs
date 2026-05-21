mod cmd;

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
        // 启动 JVM 执行主类
        start_jvm(cmd);
    }
}

/// 启动 JVM（当前仅打印解析得到的参数，作为占位实现）
fn start_jvm(cmd: Cmd) {
    println!(
        "classpath: {} class: {} args: {:?}",
        cmd.cp_option, cmd.class, cmd.args
    );
}
