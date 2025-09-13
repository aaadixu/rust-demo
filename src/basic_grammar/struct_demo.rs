// 结构体： 结构体用于创建自定义类型，字段可以包含多种数据类型。
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
pub fn struct_demo(){
    let user1 = User {
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1.username is {}",user1.username);
    println!("user1.email is {}",user1.email);
    println!("user1.sign_in_count is {}",user1.sign_in_count);
    println!("user1.active is {}",user1.active);
}
