use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

#[allow(unused_variables,unused_mut)]
pub fn smart_pointers_demo(){
    // Box<T> 是 Rust 中最简单的智能指针之一，它允许在堆上分配一块内存，并将值存储在这个内存中。
    // 由于 Rust 的所有权规则，使用 Box 可以在堆上创建具有已知大小的数据。
    let b = Box::new(5);
    println!("b = {}", b);

    // Rc<T>（引用计数指针）允许多个所有者共享数据，它使用引用计数来跟踪数据的所有者数量，并在所有者数量为零时释放数据。
    // Rc<T> 适用于单线程环境下的数据共享。
    let data = Rc::new(5);
    let data_clone = Rc::clone(&data);

    // Arc<T>（原子引用计数指针）与 Rc<T> 类似，但是可以安全地在多线程环境中共享数据，因为它使用原子操作来更新引用计数。
    // Arc<T> 本质上是一个指针（引用计数智能指针），它内部的 T 是 不可变的。
    let data = Arc::new(5);
    let data_clone = Arc::clone(&data);

    // RefCell<T> 允许在运行时检查借用规则，它使用内部可变性来提供了一种安全的内部可变性模式，允许在不可变引用的情况下修改数据。
    // 但是，RefCell<T> 只能用于单线程环境。
    let data = RefCell::new(5);
    {
        let mut borrowed_data = data.borrow_mut();
        // let mut b2 = data.borrow_mut(); // panic! already borrowed: BorrowMutError
        *borrowed_data = 10;
        println!("data is {:?},borrowed_data is {:?}",data,borrowed_data); // data is RefCell { value: <borrowed> },borrowed_data is 10
    }
    println!("data is {:?}",data); // data is RefCell { value: 10 }

    // Mutex<T> 是一个互斥锁，它保证了在任何时刻只有一个线程可以访问 Mutex 内部的数据。
    let m = Mutex::new(5);
    let mut data = m.lock().unwrap();

    // RwLock<T> 是一种读取-写入锁，允许多个读取者同时访问数据，但在写入时是排他的。
    let lock = RwLock::new(5);
    let read_guard = lock.read().unwrap();

    // Weak<T> 是 Rc<T> 的非拥有智能指针，它不增加引用计数，用于解决循环引用问题。
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);

}