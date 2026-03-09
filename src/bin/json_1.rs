use rust_project::json::model::{Event, Person};
use serde_json::Result;
use serde_json::{Value, json};
#[allow(dead_code)]
fn test() {
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!("{}", john); // 序列化为字符串输出
    println!("Name: {}", john["name"]);
    println!("First phone: {}", john["phones"][0]);
}
// 基础使用：数据绑定（类似 Jackson 的 ObjectMapper）
#[allow(dead_code)]
fn main1() -> Result<()> {
    // 序列化：Rust 对象 → JSON 字符串
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        hobbies: vec!["reading".to_string(), "coding".to_string()],
    };
    let json = serde_json::to_string(&person)?;
    println!("{}", json); // {"name":"Alice","age":30,"hobbies":["reading","coding"]}

    // 反序列化：JSON 字符串 → Rust 对象
    let data = r#"
        {
            "name": "Bob",
            "age": 25,
            "hobbies": ["swimming", "gaming"]
        }"#;
    let p: Person = serde_json::from_str(data)?;
    println!("{:?}", p); // Person { name: "Bob", age: 25, hobbies: ["swimming", "gaming"] }
    Ok(())
}
//树模型（类似 Jackson 的 JsonNode）
#[allow(dead_code)]
fn main2() -> Result<()> {
    let data = r#"
        {
            "name": "Charlie",
            "age": 35,
            "address": {
                "city": "New York",
                "zip": "10001"
            }
        }"#;

    let v: Value = serde_json::from_str(data)?;
    // 访问字段
    if let Some(name) = v.get("name").and_then(|v| v.as_str()) {
        println!("name = {}", name);
    }

    // 修改值
    let mut v = v;
    v["age"] = json!(36); // 使用 json! 宏创建 Value

    // 序列化回字符串
    let updated = serde_json::to_string_pretty(&v)?;
    println!("{}", updated);
    Ok(())
}

fn main() -> Result<()> {
    let data = r#"{ "name": "meeting", "timestamp": "2025-03-09T10:00:00Z" }"#;
    let event: Event = serde_json::from_str(data)?;
    println!("{:?}", event);
    Ok(())
}
