/// 最常见的方式是通过集合的 .iter()、.iter_mut() 或 .into_iter() 方法来创建迭代器：
///
/// .iter()：返回集合的不可变引用迭代器。
///
/// .iter_mut()：返回集合的可变引用迭代器。
///
/// .into_iter()：将集合转移所有权并生成值迭代器。
pub fn create_iterator_demo(){
    // 不可变借用
    let v = vec![1, 2, 3];
    for x in v.iter(){
        println!("{}", x);
    }

    // 可变借用
    let mut v2 = vec![1, 2, 3];
    for x in v2.iter_mut() {
        *x += 1;
    }

    // 消耗所有权
    let v3 = vec![1, 2, 3];
    for x in v3.into_iter() {
        println!("{}", x);
    }
    // v3 在这里不能再使用，因为它被 move 掉了
}