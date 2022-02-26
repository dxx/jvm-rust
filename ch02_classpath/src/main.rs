mod cmd;
mod classpath;

use crate::cmd::{parse_cmd, Cmd};
use crate::classpath::{Classpath, entry::Entry};

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

    println!("classpath: {} class: {} args: {:?}", cp, cmd.class, cmd.args);

    let class_data = match cp.read_class(&cmd.class) {
        Ok(class_data) => class_data,
        Err(err) => { panic!("Could not find or load main class {}: {}", cmd.class, err); },
    };
    println!("class data: {:?}", class_data);
}
