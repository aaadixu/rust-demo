use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn thread_demo(){
    // 创建线程
    create_thread();

    // join 等待子线程完成
    join_thread();

    // move 强制所有权迁移
    move_thread();

    // 消息传递
    chan_thread();
}

/// Rust 中通过 std::thread::spawn 函数创建新线程
fn create_thread(){
    thread::spawn(spawn_function);
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 使用闭包
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/// join 方法可以使子线程运行结束后再停止运行程序
fn join_thread(){
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 0..3 {
        println!("main thread print {}", i);
    }
}

/// move 强制所有权迁移
fn move_thread(){
    let s = String::from("hello");

    let handle = thread::spawn(move || {
        println!("{}", s);
    });
    handle.join().unwrap();
}

/// 消息传递
/// Rust 中一个实现消息传递并发的主要工具是通道（channel），通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。
///
/// std::sync::mpsc 包含了消息传递的方法
fn chan_thread(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}