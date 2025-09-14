
use std::{env, fs};
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn read_file(){
    // 读取文本文件
    match read_text_file() {
        Ok(t) => {
            println!("读取到的文本内容是：{}", t);
        }
        Err(e) => {
            println!("读取失败，错误：{}", e.to_string());
        }
    }

    // 读取二进制文本
    match read_binary_file() {
        Ok(text) => {
            println!("读取到的二进制内容是：{:?}", text);
        }
        Err(e) => {
            println!("读取失败，错误：{}", e.to_string());
        }
    }

    // 读取流式文件
    read_flow_file();
}

pub fn read_text_file() -> Result<String, Error> {
    // 获取当前工作目录
    let current_path: PathBuf = env::current_dir().expect("无法获取当前目录");

    // 打印路径
    println!("当前工作目录: {:?}", current_path);
    // 拼接完整的文件路径
    let file_path = current_path.join("src").join("file").join("test.txt");
    println!("文件路径: {:?}", file_path);
    let text = fs::read_to_string(file_path)?;
    Ok(text)
}

pub fn read_binary_file() -> Result<Vec<u8>, Error> {
    // 获取当前工作目录
    let current_path: PathBuf = env::current_dir().expect("无法获取当前目录");
    // 拼接完整的文件路径
    let file_path = current_path.join("src").join("file").join("test.txt");
    let text = fs::read(file_path)?;
    Ok(text)
}

/// std::fs 模块中的 File 类是描述文件的类，可以用于打开文件，再打开文件之后，
/// 我们可以使用 File 的 read 方法按流读取文件的下面一些字节到缓冲区（缓冲区是一个 u8 数组），读取的字节数等于缓冲区的长度。
pub fn read_flow_file(){
    // 获取当前工作目录
    let current_path: PathBuf = env::current_dir().expect("无法获取当前目录");
    // 拼接完整的文件路径
    let file_path = current_path.join("src").join("file").join("test.txt");
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open(file_path).unwrap();
    file.read(&mut buffer).unwrap();
    println!("文件流：{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("文件流：{:?}", buffer);
}