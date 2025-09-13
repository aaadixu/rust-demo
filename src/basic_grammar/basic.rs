// 第一种引用方式
// use crate::basic_grammar::variable;
// use crate::basic_grammar::constant;
// use crate::basic_grammar::func;
// use crate::basic_grammar::data_type;
// use crate::basic_grammar::control_flow;
// use crate::basic_grammar::ownership;
// use crate::basic_grammar::borrowing;
// use crate::basic_grammar::struct_demo;
// use crate::basic_grammar::enum_demo;
// use crate::basic_grammar::match_demo;
// use crate::basic_grammar::error_handle;
// use crate::basic_grammar::lifestyle;
// use crate::basic_grammar::shadowing;

// 第二种引用方式
// use crate::basic_grammar::{
//     variable,
//     constant,
//     func,
//     data_type,
//     control_flow,
//     ownership,
//     borrowing,
//     struct_demo,
//     enum_demo,
//     match_demo,
//     error_handle,
//     lifestyle,
//     shadowing,
// };

// 第三种引用方式
use crate::basic_grammar::*;
pub fn basic_grammar(){
    // 变量
    variable::variable();

    // 常量
    constant::constant();

    // 函数及返回值
    let sum = func::func_add(1,2);
    println!("sum = {}", sum);

    // 基本数据类型
    data_type::data_type();

    // 控制流
    control_flow::control_flow();

    // 所有权
    ownership::ownership();

    // 借用和引用
    borrowing::borrowing();

    // 结构体
    struct_demo::struct_demo();

    // 枚举
    enum_demo::enum_demo();

    // 模式匹配
    match_demo::match_demo();

    // 错误处理
    error_handle::error_handle();

    // 借用的生命周期
    lifestyle::lifestyle();

    // 重影（Shadowing）
    shadowing::shadowing();
}


























