use getopts::{Options, ParsingStyle};
use std::env;

/// 命令行参数解析结果
pub struct Cmd {
    /// 是否显示帮助信息（-help / -h）
    pub help_flag: bool,
    /// 是否显示版本信息（-version）
    pub version_flag: bool,
    /// classpath 路径（-classpath / -cp）
    pub cp_option: String,
    /// 要执行的主类名
    pub class: String,
    /// 传递给主类的参数列表
    pub args: Vec<String>,
}

impl Cmd {
    /// 打印命令使用方法
    pub fn print_usage(&self) {
        let args: Vec<String> = env::args().collect();
        println!("Usage: {} [-options] class [args...]", args[0])
    }
}

/// 解析命令行参数并返回 Cmd 结构体
pub fn parse_cmd() -> Cmd {
    let mut cmd = Cmd {
        help_flag: false,
        version_flag: false,
        cp_option: "".to_string(),
        class: "".to_string(),
        args: vec![],
    };

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 程序名（args[0]），用于打印 usage
    let program = args[0].clone();

    let mut opts = Options::new();
    // ParsingStyle::StopAtFirstFree: 遇到第一个非选项参数后停止解析，
    //                                后续参数都作为主类的参数，不再当作选项处理
    // long_only = true: 允许使用单横线的长选项形式，如 -classpath、-cp
    let opts = opts
        .parsing_style(ParsingStyle::StopAtFirstFree)
        .long_only(true);
    // 注册支持的选项
    opts.optflag("h", "help", "Print help message");
    opts.optflag("", "version", "Print version and exit");
    opts.optopt("", "classpath", "Specify the classpath", "classpath");
    opts.optopt("", "cp", "Specify the classpath", "classpath");

    // 解析命令行参数（跳过程序名 args[0]）
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&program, opts);
            panic!("{}", f.to_string())
        }
    };

    // 处理 -help 标记
    if matches.opt_present("help") {
        cmd.help_flag = true;
    }
    // 处理 -version 标记
    if matches.opt_present("version") {
        cmd.version_flag = true;
    }
    // 处理 classpath 选项：优先 -classpath，其次 -cp
    match matches.opt_str("classpath") {
        Some(classpath) => {
            cmd.cp_option = classpath;
        }
        None => match matches.opt_str("cp") {
            Some(cp) => {
                cmd.cp_option = cp;
            }
            None => {}
        },
    }

    // 未定义的参数（非选项参数）存放在 matches.free 中
    // 第一个为主类名，剩余的为传给主类的参数
    if !matches.free.is_empty() {
        cmd.class = matches.free[0].clone();
        cmd.args = matches.free[1..].to_vec();
    }

    cmd
}

/// 打印命令的详细使用说明（含所有选项）
fn print_usage(program: &str, opts: &mut Options) {
    let brief = format!("Usage: {} [-options] class [args...]", program);
    println!("{}", opts.usage(&brief));
}
