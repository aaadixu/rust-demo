#[allow(unused_variables)]
pub fn int_type_demo() {
    // 整型
    let a: i8 = 1; // 有符号 8-bit
    let a: u8 = 1; // 无符号 8-bit
    let a: i16 = 1; // 有符号 16-bit
    let a: u16 = 1; // 无符号 16-bit
    let a: i32 = 1; // 有符号 32-bit
    let a: u32 = 1; // 无符号 32-bit
    let a: i64 = 1; // 有符号 64-bit
    let a: u64 = 1; // 无符号 34-bit
    let a: i128 = 1; // 有符号 128-bit
    let a: u128 = 1; // 无符号 128-bit
    // isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标平台，如果是 32 位架构的处理器将使用 32 位位长度整型。
    let b :isize = 2;
    let b :usize = 2;
    //整数的表述形式：
    let a = 98_222_1111; // 十进制
    let a = 0xff; // 十六进制
    let a = 0o77; // 八进制
    let a = 0b1111_0000; // 二进制
    let a = b'A';  // 字节(只能表示 u8 型)

}
