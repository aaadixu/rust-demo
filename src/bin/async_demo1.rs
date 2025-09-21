extern crate core;

mod async_demo;
use async_demo::async_demo::execute_async_task1;

/// #[tokio::main] 宏会生成一个普通的 fn main()，并在里面启动一个 Tokio 运行时，把 async main 作为任务执行
/// 等价于：
///     let rt = Runtime::new().unwrap();
///     let result = rt.block_on(async_task());
#[tokio::main]
async fn main() {
    println!("Start executing async task...");
    // 调用异步任务执行函数，并等待其完成
    execute_async_task1().await;
    println!("Async task completed!");
}