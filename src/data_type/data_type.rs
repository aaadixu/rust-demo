use crate::data_type::*;


/// Rust 默认开启 #[warn(unused_variables)]，会提醒你这个变量没有使用，避免冗余代码。
/// 解决 warning 警告：
/// 1 使用变量
/// 2  #[allow(unused_variables)] // 完全忽略所有未使用变量的警告（不推荐）
/// 3 如果变量是故意不使用，加前缀 _
#[allow(unused_variables)]
pub fn data_type_demo() {
    // 整型
    int::int_type_demo();

    // 浮点型
    float::float_type_demo();

    // bool 类型
    let a : bool = false;
    let a : bool = true;

    // 字符型
    char::char_type_demo();

    // 复合类型
    // tuple
    tuple::tuple_type_demo();
    // vector
    vector::vector_type_demo();
}
