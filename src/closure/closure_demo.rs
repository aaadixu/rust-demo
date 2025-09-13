
pub fn closure_demo(){
    // 捕获外部变量
    capture_external_variables();

    // 闭包特性
    closure_feature();
}

/// 闭包可以捕获周围环境中的变量，这意味着它可以访问定义闭包时所在作用域中的变量。
///
/// 闭包默认按引用捕获外部变量。
///
/// 使用 move 关键字可以强制按值捕获，将外部变量的所有权转移到闭包内。
///
/// 如果闭包需要修改外部变量，需显式声明为 mut 闭包。
fn capture_external_variables(){
    let s = String::from("hello");
    // 按引用捕获（默认行为，类似 &T）
    let print_s = || println!("s = {}", s);
    print_s(); // 输出: num = 5

    // 按值捕获
    // 闭包可以通过 move 关键字获取外部变量的所有权，或者通过借用的方式获取外部变量的引用。
    let take_s = move || println!("s taken = {}", s);
    take_s(); // 输出: s taken = hello
    // println!("{}", s); // 若取消注释，将报错，s 所有权被转移

    // | 情况         | 变量类型   | move 行为 | 外部可用性 |
    // | ---------- | ------ | ------- | ----- |
    // | `i32`      | Copy   | 拷贝      | 可用    |
    // | `String`   | 非 Copy | 移动      | ❌ 报错  |
    // | `Vec<i32>` | 非 Copy | 移动      | ❌ 报错  |


    // 可变借用捕获
    let mut num = 5;
    let mut change_num = || num += 1;
    change_num();
    println!("num after closure = {}", num); // 输出: num after closure = 6

    // copy 类型的 按值捕获
    let take_num = move || println!("num taken = {}", num);
    take_num(); // 输出: num taken = 5
    println!("{}", num); // 若取消注释，能正常运行，i32 是 Copy 类型，存储在栈上，按值复制不会移动所有权
}


/// 闭包根据其捕获方式自动实现了以下三个特性：
///
/// Fn: 不需要修改捕获的变量，闭包可以多次调用。
///
/// FnMut: 需要修改捕获的变量，闭包可以多次调用。
///
/// FnOnce: 只需要捕获所有权，闭包只能调用一次。
pub fn closure_feature(){
    // 闭包可以作为函数参数
    fn apply_to_value<F>(val: i32, f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(val)
    }
    let double = |x| x * 2;
    let result = apply_to_value(5, double);
    println!("Result: {}", result); // 输出: Result: 10

    // 闭包可以作为返回值:由于闭包是匿名的，我们需要使用 impl Trait 或 Box 来描述其类型。
    // 使用 impl Fn 返回闭包
    fn make_adder1(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }
    let add_five = make_adder1(5);
    println!("5 + 3 = {}", add_five(3)); // 输出: 5 + 3 = 8

    // 使用 Box<dyn Fn> 返回闭包
    fn make_adder2(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }
    let add_ten = make_adder2(10);
    println!("10 + 2 = {}", add_ten(2)); // 输出: 10 + 2 = 12

    // FnOnce: 只需要捕获所有权，闭包只能调用一次
    fn call_closure<F>(f: F)
    where
        F: FnOnce(),
    {
        f(); // 只调用一次
    }
    let name = String::from("Rust");
    // 使用 move 强制捕获所有权
    let print_name = move || println!("Hello, {}!", name);
    call_closure(print_name);
    // println!("{}", name); // 若取消注释，将报错，name 的所有权已被移动
}