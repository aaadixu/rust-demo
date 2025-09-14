use std::fs::File;
use std::io;
use std::io::Read;

pub fn error_demo() {}

/// 对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。
#[allow(dead_code)]
fn panic_demo() {
    panic!("error occured");
    // println!("Hello, Rust");
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn result_demo() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.")
        }
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    //  if let 语法可以简化 match 语法块：
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
}

/// 可恢复的错误的传递
///
#[allow(unused_variables)]
#[allow(dead_code)]
fn error_throw() {
    let r = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }

    let r = g(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}

fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) } else { Err(false) }
}

/// ? 是 Rust 的错误传播（error propagation）运算符。
///
/// 尝试从表达式里取值
///
/// 1 如果 f(i) 返回的是 Ok(v)，那么 ? 会把里面的值 v 取出来并赋给 t。
///
/// 2 如果 f(i) 返回的是 Err(e)，那么当前函数会直接返回 Err(e)，相当于把错误往上层调用者“传递”。
fn g(i: i32) -> Result<i32, bool> {
    // 要求当前函数返回值必须兼容
    // 也就是说，当前函数的返回值类型要能表达错误，通常是 Result<T, E> 或者 Option<T>。
    //
    // 如果是 Result<T, E>，? 会把 Err(e) 往上抛。
    //
    // 如果是 Option<T>，? 会把 None 往上抛。
    let t = f(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

/// 只有 std::io::Error 才有 .kind() 方法。
///
/// 对于其他错误类型（比如你自定义的错误，或者很多库里的错误），一般是通过 match 或者 thiserror / anyhow / eyre 这种库来区分和处理，不会有 kind()。
#[allow(dead_code)]
fn kind_demo() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("No such file");
            }
            _ => {
                println!("Cannot read the file");
            }
        },
    }
}

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
