#[allow(unused_variables)]
pub fn enum_demo(){
    let book = Book2::Papery(1001);
    let ebook = Book2::Electronic(String::from("url://..."));

    let book = Book3::Papery{index: 1001};

    // match 语法
    match_enum1();
    match_enum2();

    // option 枚举
    option_demo();
}

#[derive(Debug)]
#[allow(dead_code)]
enum Book1 {
    Papery, Electronic
}


// 描述两种书的不同属性（纸质书有索书号，电子书只有 URL），可以为枚举类成员添加元组
#[derive(Debug)]
#[allow(dead_code)]
enum Book2 {
    Papery(u32),
    Electronic(String),
}

// 为属性命名，可以用结构体语法：
#[derive(Debug)]
#[allow(dead_code)]
enum Book3 {
    Papery { index: u32 },
    Electronic { url: String },
}

/// 枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。
///
/// switch 语法很经典，但在 Rust 中并不支持，很多语言摒弃 switch 的原因都是因为 switch 容易存在因忘记添加 break 而产生的串接运行问题，Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
///
/// Rust 通过 match 语句来实现分支结构。
#[allow(dead_code)]  // 告诉编译器：我知道这段代码没用，不要给我报警告
#[allow(unused_variables)]
fn match_enum1 (){
    enum Book3 {
        Papery {index: u32},
        Electronic {url: String},
    }

    let book = Book3::Papery{index: 1001};
    let ebook = Book3::Electronic{url: String::from("url...")};

    match book {
        Book3::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book3::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}

/// 如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字
#[allow(dead_code)]
fn match_enum2(){
    enum Book3 {
        Papery(u32),
        Electronic {url: String},
    }
    let book = Book3::Papery(1001);

    match book {
        Book3::Papery(i) => {
            println!("{}", i);
        },
        Book3::Electronic { url } => {
            println!("{}", url);
        }
    }
}

/// Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
fn option_demo(){
    /*
    Rust 引入了 Option 枚举类：
    enum Option<T> {
        Some(T),
        None,
    }
    */
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 初始值为空的 Option 必须明确类型：
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    // Option 是一种特殊的枚举类，它可以含值分支选择：
    let t = Some(64); // 由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }
}