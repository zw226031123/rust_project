use async_recursion::async_recursion;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    let future = async {
        println!("Hello, world!");
    };
    tokio::pin!(future);
    // future.await;
    (&mut future).await;
    println!("{}", fibonacci(10).await);
    let task = get_task("db");
    let result = task.await;
    println!("{}", result);
}
#[async_recursion]
async fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

fn get_task(source: &str) -> Pin<Box<dyn Future<Output = String>>> {
    match source {
        "db" => Box::pin(form_db()),
        "api" => Box::pin(form_api()),
        _ => Box::pin(async { "unknown".to_string() }),
    }
}
async fn form_db() -> String {
    "db".to_string()
}
async fn form_api() -> String {
    "api".to_string()
}
