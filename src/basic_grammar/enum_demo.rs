// 枚举
// 枚举允许定义可能的几种数据类型中的一种。
#[derive(Debug, PartialEq, Eq)]
enum IpAddrKind {
    V4,
    V6,
}
pub fn enum_demo(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    if four == IpAddrKind::V4 {
        println!("V4");
    }
    if six == IpAddrKind::V6 {
        println!("V6");
    }
}