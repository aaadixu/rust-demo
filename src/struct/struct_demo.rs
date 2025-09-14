
pub fn struct_demo(){
    let black = Color(0, 0, 0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);

    // 结构体方法
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());

    // 多参数案例
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("{}", rect1.wider(&rect2));

    // 结构体关联函数
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);

}


// 元组结构体
struct Color(i32,i32,i32);

// 输出结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 结构体方法
// 方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。
// 结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    // 结构体关联函数
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}







