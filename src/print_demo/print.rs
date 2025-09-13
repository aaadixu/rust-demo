
pub fn print_demo() {
    hello_world();
    placeholder()
}


fn hello_world(){
    print!("Hello, world!"); // 不换行
    println!("Hello, world!"); // 换行
}

fn placeholder(){
    let a = 1;
    // Rust 中格式字符串中的占位符不是 "% + 字母" 的形式，而是一对 {}
    println!("a = {}", a);
    // 把 a 输出两遍:
    println!("a is {}, a again is {}",a,a);
    // 更好的写法
    // 在 {} 之间可以放一个数字，它将把之后的可变参数当作一个数组来访问，下标从 0 开始
    println!("a is {0}, a again is {0}",a);
    // 输出 { 或 } : 格式字符串中通过 {{ 和 }} 分别转义代表 { 和 }
    println!("{{}}")
}