
// 错误处理
// Rust 有两种主要的错误处理方式：Result<T, E> 和 Option<T>。
pub fn error_handle(){
    // 方式一 Result
    let a = 1;
    let b = 2;
    let res = divide(a,b);
    println!("res is {:?}",res);

    // 方式二 Optional
    let v = vec![10, 20, 30];
    let res1 = get_element(10,&v);
    println!("res1 is {:?}",res1);
}

// 标准库已定义
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn get_element(index: usize, vec: &Vec<i32>) -> Option<i32> {
    if index < vec.len() {
        Some(vec[index])
    } else {
        None
    }
}