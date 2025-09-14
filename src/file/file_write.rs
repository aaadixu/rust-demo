use std::path::PathBuf;
use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn write_file() {
    // 一次性写入
    write_file_once();

    // 流式写入(新建模式)
    write_file_flow_cerate();
    // 流式写入(追加模式)
    write_file_flow_append();

}

/// 一次性写入请谨慎使用！因为它会直接删除文件内容（无论文件多么大）。如果文件不存在就会创建文件。
fn write_file_once() {
    // 获取当前工作目录
    let current_path: PathBuf = env::current_dir().expect("无法获取当前目录");
    // 拼接完整的文件路径
    let file_path = current_path.join("src").join("file").join("test.txt");
    fs::write(file_path, "Hello, world!").expect("写入失败");
}

fn write_file_flow_cerate() {
    // 获取当前工作目录
    let current_path: PathBuf = env::current_dir().expect("无法获取当前目录");
    // 拼接完整的文件路径
    let file_path = current_path.join("src").join("file").join("test.txt");
    // 注意：打开的文件一定存放在可变的变量中才能使用 File 的方法！
    let mut file = File::create(file_path).unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}

fn write_file_flow_append(){
    // 获取当前工作目录
    let current_path: PathBuf = env::current_dir().expect("无法获取当前目录");
    // 拼接完整的文件路径
    let file_path = current_path.join("src").join("file").join("test.txt");
    let mut file = OpenOptions::new()
        .append(true).open(file_path).unwrap();

    file.write(b" APPEND WORD").expect("追加写入失败");
}