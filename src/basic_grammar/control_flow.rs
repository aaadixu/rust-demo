// 控制流
pub fn control_flow(){
    // if 表达式
    let num = 7;
    if num < 5 {
        println!("小于 5");
    }else {
        println!("大于等于 5");
    }

    // loop 循环：loop 是 Rust 中的无限循环，可以使用 break 退出循环
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    // while 循环
    let mut counter = 3;
    while counter != 0 {
        println!("{}!",counter);
        counter -= 1;
    }

    // for循环
    for num in 1..5{
        print!("{}!",num);
    }
}
