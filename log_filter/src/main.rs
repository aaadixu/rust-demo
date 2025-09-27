use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(required = true, short, long, help = "日志文件路径")]
    file: String,
    #[arg(
        required = true,
        short,
        long,
        num_args = 1..,
        help = "关键词（支持多个）"
    )]
    keyword: Vec<String>,
    #[arg(
        required = false,
        short,
        long,
        default_value_t = false,
        help = "忽略大小写"
    )]
    ignore_case: bool,
    #[arg(
        required = false,
        short,
        long,
        default_value_t = false,
        help = "输出时显示行号"
    )]
    line_number: bool,

    #[arg(required = false, short, long, help = "输出到文件")]
    output: String,
}

/// cargo run -p log_filter -- -f /Users/zhixu/code/rust/rust-demo/log_filter/test.log -k test -i -l
fn main() {
    // 解析入参
    let args = Args::parse();
    // 读取日志文件
    let lines = read_file(&args.file).expect("读取文件失败");

    // if 分支可以返回值，但是值的类型必须一致
    // let res = if !args.ignore_case {
    //     filter_log_without_case(&lines, &args.keyword)
    // }else {
    //     filter_log_with_case(&lines, &args.keyword)
    // };

    //  cargo run -p log_filter -- -f /Users/zhixu/code/rust/rust-demo/log_filter/test.log -k '^1.*' -i -l
    let res = filter_log_with_regex(&lines, &args.keyword);
    for (num, line) in &res {
        if args.line_number {
            println!("【{}】{line}", num + 1)
        } else {
            println!("{line}")
        }
    }
    // cargo run -p log_filter -- -f /Users/zhixu/code/rust/rust-demo/log_filter/test.log -k '^1.*' -i -l -o /Users/zhixu/code/rust/rust-demo/log_filter/output.log
    if !args.output.trim().is_empty() {
        write_file(&args.output,&res);
    }
}

/// 使用正则表达式
fn filter_log_with_regex<'a>(
    lines: &'a Vec<String>,
    keywords: &Vec<String>,
) -> Vec<(usize, &'a String)> {
    let res = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| -> bool {
            keywords.iter().any(|keyword| -> bool {
                let re = Regex::new(keyword).expect("正则编译失败");
                re.is_match(line)
            })
        })
        .collect();
    res
}

#[allow(unused)]
fn filter_log_without_case<'a>(
    lines: &'a Vec<String>,
    keywords: &Vec<String>,
) -> Vec<(usize, &'a String)> {
    let res = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| -> bool {
            keywords
                .iter()
                .any(|keyword| -> bool { line.contains(keyword) })
        })
        .collect();
    res
}

#[allow(unused)]
fn filter_log_with_case<'a>(
    lines: &'a Vec<String>,
    keywords: &Vec<String>,
) -> Vec<(usize, &'a String)> {
    lines
        .iter()
        .enumerate()
        .filter(|(_, line)| -> bool {
            keywords
                .iter()
                .any(|keyword| -> bool { line.to_lowercase().contains(&keyword.to_lowercase()) })
        })
        .collect()
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok).collect();
    Ok(lines)
}

fn write_file(ouput :&str,lines : &Vec<(usize, &String)>) {
    let mut file = File::create(ouput).expect("创建输出文件失败");
    // 遍历 Vec<String>，逐行写入
    for (_,line) in lines {
        writeln!(file, "{}", line).expect("写入失败"); // writeln! 会在行尾自动加换行符
    }
}
