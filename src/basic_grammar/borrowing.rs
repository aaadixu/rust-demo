// 借用和引用: 借用允许引用数据而不获取所有权，通过 & 符号实现。
pub fn borrowing(){
    let s = String::from("hello");
    let len = calculate_length(&s);  // 借用
    println!("The length of '{}' is {}.", s, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}