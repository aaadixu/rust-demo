

pub fn iterator_method(){

    // 迭代器方法

    // map 对每个元素应用函数
    iterator_map_method();

    // filter 保留满足条件的元素
    iterator_filter_method();

    // take 取前 n 个元素
    iterator_take_method();

    // skip 跳过前 n 个元素
    iterator_skip_method();

    // enumerate 给元素附带索引
    iterator_enumerate_method();

    // chain 链接两个迭代器
    iterator_chain_method();

    // rev 反向迭代
    iterator_rev_method();

    // 消费器方法
    // collect 收集为集合
    iterator_collect_method();

    // sum/product 求和或乘积
    iterator_sum_method();
    iterator_product_method();

    // count 元素个数
    iterator_count_method();

    // nth 返回第 n+1 个元素
    iterator_nth_method();

    // last 返回最后一个元素
    iterator_last_method();

    // find 查找第一个满足条件的元素
    iterator_find_method();
}

/// 注意：map 是惰性执行，只有在 collect() 或消费器调用时才会真正计算。
pub fn iterator_map_method(){
    let v = [1,2,3];
    let res: Vec<_> = v.iter()
        .map(|x| { x * 2})  // 每个元素 *2
        .collect();
    println!("map res is {:?}", res); // [2, 4, 6]
}

/// 注意：闭包参数是引用类型 &&x，因为 iter() 返回 &T。
pub fn iterator_filter_method(){
    let v = [1,2,3];
    let res: Vec<_> = v.iter()
        .filter(|&&x| { x % 2 == 0})  // 每个元素 *2
        .collect();
    println!("filter res is {:?}", res); // [2]
}

/// 注意：不会改变原集合，只是限制迭代器输出的数量
pub fn iterator_take_method(){
    let v = vec![1, 2, 3, 4, 5];
    let res :Vec<_> = v.iter().take(3).copied().collect();
    println!("filter res is {:?}", res); // [1, 2, 3]
}

/// 注意：结合 take 可以实现分页功能，比如 .skip(2).take(2)。
pub fn iterator_skip_method(){
    let v = vec![1, 2, 3, 4, 5];
    let res :Vec<_> = v.iter().skip(3).copied().collect();
    println!("filter res is {:?}", res); // [4, 5]
}

/// 注意：索引类型是 usize
pub fn iterator_enumerate_method(){
    let v = vec!["a", "b", "c"];
    for (i,val) in v.iter().enumerate(){
        println!("index = {}, value = {}", i, val);
    }
}

/// 注意：原始迭代器都不会被修改，只是产生一个新的迭代器
pub fn iterator_chain_method(){
    let a = vec![1, 2];
    let b = vec![3, 4];

    let combined: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("{:?}", combined); // [1, 2, 3, 4]
}

/// 注意：只有实现了 DoubleEndedIterator 的迭代器才能使用 rev()。普通范围迭代器 0..5 支持，单向流不支持。
pub fn iterator_rev_method(){
    let v = vec![1, 2, 3];
    let reversed: Vec<_> = v.iter().rev().collect();
    println!("{:?}", reversed); // [3, 2, 1]
}

/// 注意：必须指明集合类型（如 Vec<_> 或 HashSet<_>），否则编译器无法推导
pub fn iterator_collect_method(){
    let v = vec![1, 2, 3];
    let res: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("collect res is {:?}", res);
}



/// 注意：迭代器元素类型必须实现 Sum trait。
pub fn iterator_sum_method(){
    let v = vec![1, 2, 3, 4];

    let sum: i32 = v.iter().sum();

    println!("sum = {}", sum); // sum=10
}

/// 注意：迭代器元素类型必须实现 product trait。
pub fn iterator_product_method(){
    let v = vec![1, 2, 3, 4];

    let product: i32 = v.iter().product();

    println!("product = {}", product); // product=24
}


pub fn iterator_count_method(){
    let v = vec![1, 2, 3, 4, 5];
    let cnt = v.iter().filter(|&x| x % 2 == 0).count();
    println!("偶数个数 = {}", cnt); // 2
}

pub fn iterator_nth_method(){
    let v = vec![10, 20, 30, 40];
    let third = v.iter().nth(2); // 第 3 个元素
    println!("{:?}", third); // Some(30)
}

pub fn iterator_last_method(){
    let v = vec![1, 2, 3];
    let last = v.iter().last();
    println!("{:?}", last); // Some(&3)
}

/// 注意：返回的是 Option<&T>，如果没有找到返回 None。
pub fn iterator_find_method(){
    let v = vec![1, 2, 3, 4, 5];
    let first_even = v.iter().find(|&x| x % 2 == 0);
    println!("find res is {:?}", first_even); // Some(&2)
}