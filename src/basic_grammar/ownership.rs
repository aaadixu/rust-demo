// 所有权
// Rust 中的所有权是独特的内存管理机制，核心概念包括所有权 (ownership)、借用 (borrowing) 和引用 (reference)。
// 所有权规则:
// 1 Rust 中的每个值都有一个所有者。
// 2 每个值在任意时刻只能有一个所有者。
// 3 当所有者超出作用域时，值会被删除。
pub fn ownership(){
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被转移给了 s2
    // println!("{}", s1); // 此处编译会报错，因为 s1 已不再拥有该值
    println!("{}",s2);
}