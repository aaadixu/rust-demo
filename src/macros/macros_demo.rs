
/// 用 macro_rules! 关键字来定义声明式宏
/// 宏的定义是基于模式匹配的，可以匹配代码的结构并根据匹配的模式生成相应的代码。
/// 模式匹配：宏通过模式匹配来匹配传递给宏的代码片段，模式是宏规则的左侧部分，用于捕获不同的代码结构。
/// 规则：宏规则是一组由 $ 导的模式和相应的展开代码，规则由分号分隔。
/// 宏的展开：当宏被调用时，匹配的模式将被替换为相应的展开代码，展开代码是宏规则的右侧部分。
pub fn macros_demo(){

    // expr 宏的定义
    create_expr_macro();

    // ident 宏的定义
    create_ident_macro();

    // ty 宏的定义
    create_ty_macro();

    // pat 宏的定义
    create_pat_macro();

    // tt 宏的定义
    create_tt_macro();

    // tt 宏的定义
    create_stmt_macro();

    // block 宏的定义
    create_block_macro();

    // path 宏的定义
    create_path_macro();
}


// expr 宏的定义
macro_rules! print_expr {
    // 模式匹配
    ($e:expr) => {
        // 宏的展开
        println!("表达式的值是: {}!", $e);
    };
}


/// expr → 表达式
///
/// 匹配任何表达式，比如字面量、变量、函数调用等。
fn create_expr_macro(){
    print_expr!(1 + 2);       // 表达式: 1+2
    print_expr!("hello");     // 表达式: "hello"
    let x = 42;
    print_expr!(x * 2);       // 表达式: x*2
}


macro_rules! say_hello {
    ($name:ident) => {
        fn $name() {
            println!("Hello from {}", stringify!($name));
        }
    };
}

/// ident → 标识符
///
/// 匹配变量名、函数名、类型名等 标识符。
fn create_ident_macro(){
    say_hello!(rust_func);
    rust_func(); // 调用宏生成的函数
}

macro_rules! create_vec {
    ($t:ty) => {
        Vec::<$t>::new()
    };
}

/// ty → 类型
///
/// 匹配一个类型。
fn create_ty_macro() {
    let v: Vec<i32> = create_vec!(i32);
    println!("vec length = {}", v.len());
}

macro_rules! match_value {
    ($val:expr, $p:pat) => {
        match $val {
            $p => println!("匹配成功!"),
            _ => println!("匹配失败!"),
        }
    };
}

fn create_pat_macro() {
    match_value!(Some(5), Some(_x)); // 匹配成功
    // match_value!(None, Some(_));    // 匹配失败
}



macro_rules! dump_tt {
    ($($t:tt)*) => {
        println!("token tree: {}", stringify!($($t)*));
    };
}
/// tt → Token Tree（词法单元树）
///
/// 几乎匹配任何内容，最通用。
fn create_tt_macro() {
    dump_tt!(abc);        // 标识符
    dump_tt!(1 + 2);      // 表达式
    dump_tt!({ foo });    // 代码块
}


macro_rules! wrap_block {
    ($s:stmt) => {
        {
            println!("执行语句前");
            $s
            println!("执行语句后");
        }
    };
}

/// stmt → 语句
///
/// 匹配一个语句。
fn create_stmt_macro() {
    wrap_block!(let _x = 10);
    wrap_block!(println!("x = {}", 10));
}

macro_rules! run_block {
    ($b:block) => {
        {
            println!("运行代码块");
            $b
        }
    };
}
/// block → 代码块
///
/// 匹配 { ... } 形式的语法块。
fn create_block_macro() {
    run_block!({
        let x = 99;
        println!("x = {}", x);
    });
}


macro_rules! call_func {
    ($p:path) => {
        {
            println!("调用路径: {}", stringify!($p));
            $p()
        }
    };
}

mod utils {
    pub fn hello() {
        println!("Hello from utils!");
    }
}

/// path → 路径
///
/// 匹配模块/类型/函数路径。
fn create_path_macro() {
    call_func!(utils::hello);
}
