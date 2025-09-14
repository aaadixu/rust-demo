#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn vector_demo(){
    // 空的堆分配动态数组，需后续 push
    let vector: Vec<i32> = Vec::new(); // 创建类型为 i32 的空向量
    // 带初始值的堆分配动态数组
    let vector = vec![1, 2, 4, 8];     // 通过数组创建向量
    // 固定大小的栈分配数组，大小不可变
    let vector = [1,2,3,4];

    // 向量只有 push 方法来追加单个元素：
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    // append 方法用于将一个向量拼接到另一个向量的尾部：
    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("v1 after append:{:?}", v1); // [1, 2, 4, 8, 16, 32, 64]
    println!("v2 after append:{:?}", v2);  // []

    // get 方法用于取出向量中的值：
    let mut v = vec![1, 2, 4, 8];
    println!("{}", match v.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });

    // 因为向量的长度无法从逻辑上推断，get 方法无法保证一定取到值，所以 get 方法的返回值是 Option 枚举类，有可能为空。
    //
    // 这是一种安全的取值方法，但是书写起来有些麻烦。如果你能够保证取值的下标不会超出向量下标取值范围，你也可以使用数组取值语法：
    let v = vec![1, 2, 4, 8];
    println!("{}", v[1]);

    // 遍历向量：
    let v = vec![100, 32, 57];
    // 借用迭代
    for i in &v {
        println!("{}", i); // i 是 &i32
    }
    println!("v 还可以用: {:?}", v); // ✅ 可用

    // 移动迭代
    for i in v {
        println!("{}", i); // i 是 i32
    }
    // println!("{:?}", v); // ❌ v 已被移动，不能再用
}