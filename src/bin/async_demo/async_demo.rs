use tokio;
use tokio::time::{self, Duration};
use std::error::Error;
use std::{fmt, io};
use tokio::fs::File;
use reqwest::get;
use async_std::task;
use tokio::io::AsyncReadExt;
use tokio::spawn;
use tokio::sync::mpsc;

// 异步函数，模拟异步任务
async fn async_task() -> u32 {
    // 模拟异步操作，等待 1 秒钟
    time::sleep(Duration::from_secs(1)).await;
    // 返回结果
    42
}

// 异步任务执行函数
#[allow(unused)]
pub async fn execute_async_task1() {
    // 调用异步任务，并等待其完成
    let result = async_task().await;
    // 输出结果
    println!("Async task result: {}", result);
}


// 异步函数，用于执行 HTTP GET 请求并返回响应结果
async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    // 使用 reqwest 发起异步 HTTP GET 请求
    let response = get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// 异步任务执行函数
pub async fn execute_async_task2() -> Result<(), Box<dyn Error>> {
    // 发起异步 HTTP 请求
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let result = fetch_url(url).await?;
    // 输出响应结果
    println!("Response: {}", result);
    Ok(())
}


/// async 关键字
/// async 关键字用于定义异步函数，即返回 Future 或 impl Future 类型的函数。异步函数执行时会返回一个未完成的 Future 对象，它表示一个尚未完成的计算或操作。
///
/// 异步函数可以包含 await 表达式，用于等待其他异步操作的完成。
#[allow(unused)]
async fn hello() -> String {
    "Hello, world!".to_string()
}

/// await 关键字
///
/// await 关键字用于等待异步操作的完成，并获取其结果。
///
/// await 表达式只能在异步函数或异步块中使用，它会暂停当前的异步函数执行，等待被等待的 Future 完成，然后继续执行后续的代码。
#[allow(unused)]
async fn print_hello() {
    let result = hello().await;
    println!("{}", result);
}


/// 异步函数返回值
///
/// 异步函数的返回值类型通常是 impl Future<Output = T>，其中 T 是异步操作的结果类型。由于异步函数的返回值是一个 Future，因此可以使用 .await 来等待异步操作的完成，并获取其结果。
#[allow(unused)]
async fn add(a: i32, b: i32) -> i32 {
    a + b
}


/// 异步块
///
/// 除了定义异步函数外，Rust 还提供了异步块的语法，可以在同步代码中使用异步操作。异步块由 async { } 构成，其中可以包含异步函数调用和 await 表达式。
#[allow(unused)]
fn test_block(){
    async {
        let result1 = hello().await;
        let result2 = add(1, 2).await;
        println!("Result: {}, {}", result1, result2);
    };
}

/// 异步任务执行
///
/// 在 Rust 中，异步任务通常需要在执行上下文中运行，可以使用 tokio::main、async-std 的 task::block_on 或 futures::executor::block_on 等函数来执行异步任务。这些函数会接受一个异步函数或异步块，并在当前线程或执行环境中执行它。
#[allow(unused)]
fn test_async(){
    task::block_on(print_hello());
}


// 自定义错误类型
#[derive(Debug)]
pub struct MyError {
    msg: String,
}

impl MyError {
    pub fn new(msg: &str) -> Self {
        MyError { msg: msg.to_string() }
    }
}

// 实现 Display trait
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.msg)
    }
}
impl Error for MyError {}

async fn some_async_operation() -> Result<(), MyError> {
    // 模拟出错
    Err(MyError::new("Something went wrong"))
}
/// 错误处理
///
/// await 后面跟一个 ? 操作符可以传播错误。如果 await 的 Future 完成时返回了一个错误，那么这个错误会被传播到调用者。
#[allow(unused)]
async fn my_async_function() -> Result<(), MyError> {
    some_async_operation().await?;
    // 如果 some_async_operation 出错，错误会被传播
    println!("Operation succeeded!");
    Ok(())
}

/// 异步 trait 方法
///
/// Rust 允许为 trait 定义异步方法。这使得你可以为不同类型的对象定义异步操作。
#[allow(unused)]
trait MyAsyncTrait {
    async fn async_method(&self) -> Result<(), MyError>;
}

#[allow(unused)]
type MyType = String;

impl MyAsyncTrait for MyType {
    async fn async_method(&self) -> Result<(), MyError> {
        // 异步逻辑
        Ok(())
    }
}


/// 异步宏
///
/// Rust 提供了一些异步宏，如 tokio::spawn，用于在异步运行时中启动新的异步任务。
#[allow(unused)]
async fn main() -> io::Result<()> {
    let mut file = File::open("file.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("Contents: {}", contents);
    Ok(())
}

/// 异步通道
///
/// Rust 的一些异步运行时提供了异步通道（如 tokio::sync::mpsc），允许在异步任务之间传递消息。
#[allow(unused)]
async fn mpsc_demo() {
    let (tx, mut rx) = mpsc::channel(32);

    let child = spawn(async move {
        let response = "Hello, world!".to_string();
        tx.send(response).await.unwrap();
    });

    let response = rx.recv().await.unwrap();
    println!("Received: {}", response);

    child.await.unwrap();
}