use csv::ReaderBuilder;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::File;
#[allow(dead_code)]
pub fn test_main() {
    let client = Client::new();
    let result = client
        .post("https://app.shb.ltd/api/middleware/outside/moen/moenTaskMileage")
        .header("token", "2cc11e438c2017a7361c301480266c78_41")
        .json(&json!({"taskId": "19587e20-8b8f-11f0-b1ea-00163e0b3979"}))
        .send()
        .expect("request error");

    println!("Result: {:?}", result);
}
#[derive(Deserialize, Serialize)]
struct MoenTaskMileage {
    #[serde(rename = "taskId")]
    task_id: String,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./task.txt")?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // 关键设置：文件没有标题行
        .from_reader(file);
    let client = Client::new();
    for result in rdr.records() {
        let record = result?;
        // 此时 record[0] 就是第一列的数据，而不是列名
        let split = record[0].split("\t").collect::<Vec<&str>>();
        let id = split.get(0).unwrap();
        let task_no = split.get(1).unwrap();
        // https://app.shb.ltd/api/middleware/outside/moen/moenTaskMileage

        let mileage = MoenTaskMileage {
            task_id: id.to_string(),
        };
        let result = client
            .post("https://app.shb.ltd/api/middleware/outside/moen/moenTaskMileage")
            .header("token", "2cc11e438c2017a7361c301480266c78_41")
            .json(&mileage)
            .send()
            .expect("request error")
            .text();
        let str: String = match result {
            Err(_e) => String::from("错误"),
            Ok(text) => {
                let json: Value = serde_json::from_str(&text)?;
                // 使用 get() 方法链式访问
                // 注意：get() 返回 Option，需要 unwrap() 或 unwrap_or()
                // let option = json.get("data").cloned()
                //     .and_then(|d| d.get("taskMileage").cloned())
                //     .unwrap()
                //     .as_str();
                let data_value = json.get("data").cloned();
                // 或者直接提取你需要的数值
                let option: Option<String> = data_value
                    .and_then(|d| d.get("taskMileage").cloned())
                    .and_then(|v| Option::from(v.to_string()));
                option.unwrap_or_else(|| String::from(""))
            }
        };
        println!("{}\t\t{}", task_no, str);
    }
    Ok(())
    // AZ2025091702814 AZ2025091702816 WX2025091706379
}

pub fn test() {
    println!("test");
}
