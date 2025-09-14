#[allow(dead_code)]
#[allow(unused)]
pub fn string_demo() {
    // 新建字符串
    let string = String::new();

    // 基础类型转换成字符串
    let one = 1.to_string(); // 整数到字符串
    let float = 1.3.to_string(); // 浮点数到字符串
    let slice = "slice".to_string(); // 字符串切片到字符串

    // 包含 UTF-8 字符的字符串
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // 字符串追加 所有权不会变
    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!'); // 追加字符

    // 字符串拼接
    str_concat();

    // 字符串长度
    str_len();
}

#[allow(unused)]
fn str_concat() {
    // 用 + 号拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // 这个语法也可以包含字符串切片
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // s1、s2、s3 都是 String 类型，存储在堆上，变量自身在栈上存指针、长度和容量。
    // 左边 s1 是 String，会 被移动（所有权转移到 add 内部），右边 "-" 是 &str，借用它，不会移动
    // 执行后：s1 不再可用，返回一个新的 String，内容 "tic-"
    // + 运算符对于 String 定义为：fn add(self, s: &str) -> String

    // 总结：
    // String + &str 会移动左边的 String，右边可以是借用
    //
    // 如果连续使用 +，左边每次都是新的 String，左边的变量会被移动
    //
    // 右边使用 &String → 自动解引用为 &str，所有权不变
    let s = s1 + "-" + &s2 + "-" + &s3;

    // 使用 format! 宏
    // s1、s2、s3 都 不会被移动
    // format! 会返回一个新的 String
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = "tic";
    let s2 = "tac";
    let s3 = "toe";
    // 实际上是 不能编译的。原因在于：
    // "tic"、"tac"、"toe" 这些是 字符串字面量，类型是 &'static str，而不是 String。
    // 也就是说：左边必须是 String，并且会发生 所有权转移（move）。右边必须是 &str。
    // let s = s1 + "-" + &s2 + "-" + &s3; // 无法编译
}


#[allow(unused)]
fn str_len(){
    let s = "hello";
    let len = s.len();  // 5

    // 中文是 UTF-8 编码的，每个字符长 3 字节，所以长度为6
    let s = "你好";
    let len = s.len(); // 6

    // 如果想统计字符数量可以先取字符串为字符集合(统计字符的速度比统计数据长度的速度慢得多)：
    let s = "hello你好";
    let len = s.chars().count(); // 7
}