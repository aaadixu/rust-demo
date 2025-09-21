extern crate core;

mod async_demo;

use tokio::runtime::Runtime;
use async_demo::async_demo::execute_async_task2;


 fn main() {
    // 创建异步运行时
    let rt = Runtime::new().unwrap();
    // 在异步运行时中执行异步任务
    let result = rt.block_on(execute_async_task2());
    // 处理异步任务执行结果
    match result {
        Ok(_) => println!("Async task executed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}