mod print_demo;
mod basic_grammar;
mod data_type;
mod iterator;
mod closure;
mod r#struct;
mod r#enum;

use print_demo as printdemo;
use data_type::data_type as datatype;
use iterator::iterator_demo;
use closure::closure_demo;
use r#struct::struct_demo;
use r#enum::enum_demo;

fn main() {
    // rust 输出到命令行
    printdemo::print::print_demo();

    // 基础语法
    basic_grammar::basic::basic_grammar();

    // 数据类型
    datatype::data_type_demo();

    // 迭代器
    iterator_demo::iterator_demo();

    // 闭包
    closure_demo::closure_demo();

    // 结构体
    struct_demo::struct_demo();
    
    // 枚举
    enum_demo::enum_demo();
}
