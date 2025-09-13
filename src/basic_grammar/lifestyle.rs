
// 所有权与借用的生命周期
// Rust 使用生命周期来确保引用的有效性。生命周期标注用 'a 等来表示，但常见的情况下，编译器会自动推导。
pub fn lifestyle(){
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    let res = longest(&s1,&s2);
    println!("longest res is {}",res)
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
