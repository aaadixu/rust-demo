mod print_demo;
mod basic_grammar;
mod data_type;
mod iterator;
mod closure;
mod r#struct;
mod r#enum;
mod error;
mod file;
mod set_data_type;
mod concurrent;

use print_demo as printdemo;
use data_type::data_type as datatype;
use iterator::iterator_demo;
use closure::closure_demo;
use r#struct::struct_demo;
use r#enum::enum_demo;
use error::error_demo;
use clap::{Parser,ValueEnum};
use file::file_demo;
use set_data_type::set_data_type as setdata;
use concurrent::thread;
/// 命令行参数示例
#[derive(Parser, Debug)]
#[command(author, version, about)] // clap 只支持 author、version、about、long_about 等字段
struct Args {
    /// 用户名 (必填)
    #[arg(short, long,default_value = "dixu", help = "用户名")]
    username: String,

    /// 端口号 (可选，默认值 8080)
    #[arg(short, long, default_value_t = 8080, help = "端口号")]
    port: u16,

    /// 调试模式 (布尔 flag)
    #[arg(short, long, default_value_t = false, help = "开启调试模式")]
    debug: bool,

    /// 支持多值参数 (可以指定多个文件)
    #[arg(short, long, num_args = 1.., help = "文件列表")]
    files: Vec<String>,

    /// 枚举类型参数
    #[arg(long, value_enum, ignore_case = true, default_value_t = Mode::Fast, help = "运行模式")]
    mode: Mode,
}

/// 枚举类型参数
#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Fast,
    Slow,
}

fn main() {
    // Rust 中主函数是个无参函数，环境参数需要开发者通过 std::env 模块取出
    let args = std::env::args();
    // cargo run -- a b
    // 输出：Args { inner: ["target/debug/rust-demo", "a", "b"] }
    println!("{:?}", args);

    // # 运行示例
    // cargo run -- -u Alice -f file1.txt file2.txt --mode Slow -d
    let args = Args::parse();
    println!("username = {}", args.username);
    println!("port = {}", args.port);
    println!("debug = {}", args.debug);
    println!("files = {:?}", args.files);
    println!("mode = {:?}", args.mode);

    // rust 输出到命令行
    printdemo::print::print_demo();

    // 基础语法
    basic_grammar::basic::basic_grammar();

    // 数据类型
    datatype::data_type_demo();

    // 迭代器
    iterator_demo::iterator_demo();

    // 闭包
    closure_demo::closure_demo();

    // 结构体
    struct_demo::struct_demo();
    
    // 枚举
    enum_demo::enum_demo();
    
    // 异常处理
    error_demo::error_demo();

    // 文件读写
    file_demo::file_demo();

    // 集合类型数据
    setdata::set_data_type_demo();

    // 并发
    thread::thread_demo();
}
