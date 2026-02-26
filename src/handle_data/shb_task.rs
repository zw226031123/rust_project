use crate::error::project_error::ProjectError;
use crate::handle_data::model::{Request, TaskRecordListParam};
use reqwest::blocking::Client;

pub fn search_task(request: &Request, body_str: String) -> Result<String, ProjectError> {
    let client = Client::new();
    let result = client
        .post(format!(
            "{}/api/elasticsearch/outside/es/task/search",
            request.host
        ))
        .header("token", &request.token)
        .body(body_str) // ⬅️ 关键方法：直接放入请求体
        .header("Content-Type", "application/json") // 手动设置头（如果是JSON）
        .send()
        .expect("request error")
        .text();
    Ok(result.unwrap())
}

pub fn task_record_list(
    request: &Request,
    param: TaskRecordListParam,
) -> Result<String, ProjectError> {
    let client = Client::new();
    let result = client
        .get(format!(
            "{}/api/task/outside/pc/taskrecord/list",
            request.host
        ))
        .header("token", &request.token)
        .query(&param) // ⬅️ 关键方法：直接放入请求体
        .header("Content-Type", "application/json") // 手动设置头（如果是JSON）
        .send()
        .expect("request error")
        .text();
    Ok(result.unwrap())
}
