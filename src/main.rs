mod print_demo;
mod basic_grammar;
mod data_type;
mod iterator;

use print_demo as printdemo;
use data_type::data_type as datatype;
use iterator::iterator_demo;

fn main() {
    // rust 输出到命令行
    printdemo::print::print_demo();
    
    // 基础语法
    basic_grammar::basic::basic_grammar();
    
    // 数据类型
    datatype::data_type_demo();
    
    // 迭代器
    iterator_demo::iterator_demo();
}
