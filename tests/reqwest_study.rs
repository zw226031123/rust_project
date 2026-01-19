use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::blocking::Client;
use reqwest::{Client as AsyncClient};
use rust_project::models::flink::Flink;
use serde_json::Value;
use std::time::Duration;
use tokio;
// 引入tokio库

#[test]
fn main_sync() {
    let client = Client::new();
    let authorization = base64("publink", "rxe3N@9%");
    // let url = "http://flink-console-test.shb.ltd/jobs/overview";
    let url="http://flink-console.linker.ltd/jobs/overview";
    let response_result = client
        .get(url)
        .header("authorization", authorization)
        .send();
    match response_result {
        Err(e) => println!("Failed to execute request,{:?}", e),
        Ok(response) => {
            let body = response.text();
            match body {
                Err(e) => println!("response text error, {:?}", e),
                Ok(body) => {
                    println!("Response body: {}", body);
                    let flink: Value =
                        serde_json::from_str(&body).expect("Failed to read response");
                    let jobs_str = flink.get("jobs").unwrap().to_string();
                    let job_list: Vec<Flink> =
                        serde_json::from_str(&jobs_str).expect("Failed to read response");
                    job_list.iter().for_each(|job| {
                        println!(
                            "name:{},state:{},failed:{}",
                            job.name, job.state, job.tasks.failed
                        );
                    })
                }
            }
        }
    }
}
#[tokio::test]
async fn main() {
    // 发送GET请求
    let client = AsyncClient::new();
    let authorization = base64("publink", "rxe3N@9%");
    let resp = client.get("http://flink-console.linker.ltd/jobs/overview")
        .header("authorization", authorization)
        .send()
        .await
        .unwrap() // 等待请求完成，并处理错误
        .text() // 将响应体转换为文本
        .await
        .unwrap(); // 等待文本转换完成，并处理错误
    println!("Response: {}", resp);
    let flink: Value =
        serde_json::from_str(&resp).expect("Failed to read response");
    let jobs_str = flink.get("jobs").unwrap().to_string();
    let job_list: Vec<Flink> =
        serde_json::from_str(&jobs_str).expect("Failed to read response");
    job_list.iter().for_each(|job| {
        println!(
            "name:{},state:{},failed:{}",
            job.name, job.state, job.tasks.failed
        );
    })
}

fn base64(username: &str, password: &str) -> String {
    let authorization = format!(r#"{}:{}"#, username, password);
    let mut result = String::from("Basic ");
    result.push_str(BASE64_STANDARD.encode(authorization).as_str());
    result
}
// #[test]
#[tokio::test]
async fn test() {
    let rc = std::sync::Arc::new(tokio::sync::Mutex::new(false));
    let executed = rc.clone();
    let _join_handle = tokio::spawn(async move {
        println!("task running");
        *executed.lock().await = true;
        std::future::pending::<()>().await;
    });
    let executed = rc.clone();
    loop {
        let v = *executed.lock().await;
        println!("{v}");
        trpl::sleep(Duration::from_millis(2000)).await;
    }
}
