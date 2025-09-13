// Rust 是静态类型语言，在变量声明时可以显式指定类型，但通常可以依赖类型推断
// 基本类型: i32 (32位有符号整数), u32 (32位无符号整数), f64 (64位浮点数), bool (布尔类型), char (字符)
pub fn data_type(){
    let x: i32 = 42;
    let y: f64 = 3.14;
    let is_true: bool = true;
    let letter: char = 'A';
    println!("x is {}, y is {}, is_true is {},letter is {}",x,y,is_true,letter)
}
