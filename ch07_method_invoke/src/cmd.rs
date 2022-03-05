use std::env;
use getopts::{Options, ParsingStyle};

#[derive(Debug)]
pub struct Cmd {
    pub help_flag: bool,
    pub version_flag: bool,
    pub cp_option: String,
    pub x_jre_option: String,
    pub class: String,
    pub args: Vec<String>,
}

impl Cmd {
    pub fn print_usage(&self) {
        let args: Vec<String> = env::args().collect();
        println!("Usage: {} [-options] class [args...]", args[0])
    }
}

pub fn parse_cmd() -> Cmd {
    let mut cmd = Cmd{
        help_flag: false,
        version_flag: false,
        cp_option: "".to_string(),
        x_jre_option: "".to_string(),
        class: "".to_string(),
        args: vec![],
    };

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let mut opts = Options::new();
    // ParsingStyle::StopAtFirstFree: 解析时剩余参数不作为标记参数一部分
    // long_only = true: 允许使用 -xxx
    let opts = opts.parsing_style(ParsingStyle::StopAtFirstFree).long_only(true);
    opts.optflag("h", "help", "Print help message");
    opts.optflag("", "version", "Print version and exit");
    opts.optopt("", "classpath", "Specify the classpath", "classpath");
    opts.optopt("", "cp", "Specify the classpath", "classpath");
    opts.optopt("", "Xjre", "Path to jre", "jre");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts);
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("help") {
        cmd.help_flag = true;
    }
    if matches.opt_present("version") {
        cmd.version_flag = true;
    }
    match matches.opt_str("classpath") {
        Some(classpath) => {
            cmd.cp_option = classpath;
        },
        None => {
            match matches.opt_str("cp") {
                Some(cp) => {
                    cmd.cp_option = cp;
                },
                None => {}
            }
        }
    }
    match matches.opt_str("Xjre") {
        Some(jre) => {
            cmd.x_jre_option = jre;
        },
        None => {}
    }

    // 未定义的参数放在 free Vec 中
    if !matches.free.is_empty() {
        cmd.class = matches.free[0].clone();
        cmd.args = matches.free[1..].to_vec();
    }

    cmd
}

fn print_usage(program: &str, opts: &mut Options) {
    let brief = format!("Usage: {} [-options] class [args...]", program);
    println!("{}", opts.usage(&brief));
}
