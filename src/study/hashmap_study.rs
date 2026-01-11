use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("a"), String::from("a"));
    map.insert(String::from("b"), String::from("b"));
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let v_tuple = vec![("a", "a"), ("b", "b"), ("c", "c")];
    let v_map: HashMap<_, _> = v_tuple.into_iter().collect();

    let option = v_map.get("a").copied().unwrap_or("");
    println!("{}", option);

    let field_name = String::from("c");
    let field_value = String::from("c");
    map.insert(field_name, field_value);
    // println!("{field_name}-{field_value}");

    //更新
    map.insert(String::from("c"), String::from("d"));

    //不存在 添加
    map.entry(String::from("c")).or_insert(String::from("e"));

    //更新
    let text = "hello world wonderful world";
    let mut word_map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        //更新
        *count += 1;
    }
    println!("{:?}", word_map);
}
