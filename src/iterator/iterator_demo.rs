use crate::iterator::*;
pub fn iterator_demo(){
    // 循环 - loop
    loop_demo::loop_demo();
    // 创建迭代器
    create_iterator::create_iterator_demo();

    // 迭代器方法
    iterator_method::iterator_method();

    // 自定义迭代器
    customize_iterator::customize_iterator_demo();
}