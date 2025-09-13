// 模式匹配 (match)
// match 是 Rust 中强大的控制流工具，类似于 switch 语句。
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
pub fn match_demo(){
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("value is {}",value);
    println!("value is {}",value_in_cents(Coin::Nickel));
    println!("value is {}",value_in_cents(Coin::Dime));
    println!("value is {}",value_in_cents(Coin::Quarter));
}