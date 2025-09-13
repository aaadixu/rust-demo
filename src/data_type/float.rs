
/// Rust 与其它语言一样支持 32 位浮点数（f32）和 64 位浮点数（f64）。
///
/// 默认情况下，64.0 将表示 64 位浮点数，因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。
///
/// Rust 不支持 ++ 和 --，因为这两个运算符出现在变量的前后会影响代码可读性，减弱了开发者对变量改变的意识能力
#[allow(unused_variables)]
pub fn float_type_demo() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
}