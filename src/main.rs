mod print_demo;
mod basic_grammar;

use print_demo as printdemo;


fn main() {
    // rust 输出到命令行
    printdemo::print::print_demo();
    // 基础语法
    basic_grammar::basic::basic_grammar();
}
