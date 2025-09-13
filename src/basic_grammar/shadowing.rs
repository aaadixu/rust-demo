// 重影（Shadowing）
// 重影的概念与其他面向对象语言里的"重写"（Override）或"重载"（Overload）是不一样的。重影就是刚才讲述的所谓"重新绑定"，之所以加引号就是为了在没有介绍这个概念的时候代替一下概念。
// 重影就是指变量的名称可以被重新使用的机制

pub fn shadowing(){
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

