use std::collections::HashMap;

pub fn map_demo(){
    let mut map:HashMap<&str,&str> = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    println!("color = {}", map.get("color").unwrap());

    // 映射表支持迭代器：
    for p in map.iter() {
        println!("{:?}", p);
    }

    // 如果没有键为 "color" 的键值对就添加它并设定值为 "red"，否则将跳过
    map.entry("color").or_insert("red");

    // 在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法：
    let mut map = HashMap::new();
    map.insert(1, "a");

    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
}