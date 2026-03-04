use crate::error::project_error::ProjectError;
use crate::handle_data::model::{
    AmapDrivingDistanceResponsePath, EstimatedDistanceParam, Request, Task, TaskRecord,
    TaskRecordListParam,
};
use csv::ReaderBuilder;
use reqwest::blocking::Client;
use rust_decimal::{Decimal, RoundingStrategy};
use serde_json::Value;
use std::fs::File;
use std::str::FromStr;

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
pub fn estimated_distance(
    start_position_longitude: Decimal,
    start_position_latitude: Decimal,
    end_position_longitude: Decimal,
    end_position_latitude: Decimal,
) -> Result<Decimal, ProjectError> {
    let client = Client::new();
    let query = EstimatedDistanceParam {
        key: String::from("3cf1b829475ac81c406ceca9ec57d73d"),
        strategy: 11,
        origin: format!("{},{}", start_position_longitude, start_position_latitude),
        destination: format!("{},{}", end_position_longitude, end_position_latitude),
    };
    let result = client
        .get("https://restapi.amap.com/v3/direction/driving")
        .query(&query)
        .send()
        .expect("request error")
        .text();
    let result = match result {
        Err(_) => Vec::new(),
        Ok(body) => {
            println!("Response body: {}", body);
            let data: Value = serde_json::from_str(&body).expect("Failed to read response");
            let paths = data.get("route").unwrap().get("paths").unwrap().to_string();
            let result: Vec<AmapDrivingDistanceResponsePath> =
                serde_json::from_str(&paths).expect("Failed to read response");
            result
        }
    };
    if result.is_empty() {
        return Err(ProjectError::Null());
    }
    let result_distance: &String = if result.len() > 2 {
        &result.get(1).unwrap().distance
    } else {
        &result.get(0).unwrap().distance
    };
    //千
    let one_thousand = Decimal::new(1000, 0);
    let result = Decimal::from_str(result_distance.as_str())? / one_thousand;
    Ok(result.round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero))
}

pub fn task_mileage(task_no: String, task_id: String) -> Result<Decimal, ProjectError> {
    let request = Request {
        token: String::from("a498487e9f40741dcc7eb1bb317529e0_41"),
        host: String::from("https://app.shb.ltd"),
    };
    let body_str = format!(
        r#"{{"page":1,"pageSize":100,"queryCreateHalf":true,"keyword":"{task_no}","templateId":"","stateList":[],"isException":null,"exceptionStates":[],"exceptionNodes":[],"conditions":[],"systemConditions":[],"createUser":"","executor":"","synergyId":"","currentNodeExecutorUser":"","nodeProcessedUser":"","searchNodeId":"","whoseInfo":"my","privacy":true}}"#
    );
    let task_result = search_task(&request, body_str);
    if let Ok(task_result) = task_result {
        // println!("Response body: {}", res);

        let task_content = serde_json::from_str::<Value>(&task_result)
            .expect("Failed to read response")
            .get("result")
            .unwrap()
            .get("content")
            .unwrap()
            .to_string();
        let task_vec: Vec<Task> =
            serde_json::from_str(&task_content).expect("Failed to read response");
        let task = task_vec.iter().find(|task| task.id == task_id);
        if let Some(task) = task {
            let param = TaskRecordListParam {
                task_id,
                page_num: 1,
                page_size: 100,
                user_id: String::from("89cb970f4f38de533412e625030446fc"),
                has_view_balance_record: 1,
                has_view_review_record: 1,
            };
            let task_record_result = task_record_list(&request, param);
            if let Ok(task_record_result) = task_record_result {
                let task_record_json: Value =
                    serde_json::from_str(&task_record_result).expect("Failed to read response");
                let task_record_list = task_record_json
                    .get("result")
                    .unwrap()
                    .get("list")
                    .unwrap()
                    .to_string();
                let task_record_vec: Vec<TaskRecord> =
                    serde_json::from_str(&task_record_list).expect("Failed to read response");
                let task_record = task_record_vec.iter().find(|task_record| {
                    task_record.action.eq("出发")
                        && task_record.latitude.is_none()
                        && task_record.longitude.is_none()
                });
                if let Some(task_record) = task_record {
                    let task_distance = estimated_distance(
                        task_record.longitude.unwrap(),
                        task_record.latitude.unwrap(),
                        task.address.longitude,
                        task.address.latitude,
                    );
                    if let Ok(task_distance) = task_distance {
                        println!("{}\t\t{}", task_no, task_distance);
                        return Ok(task_distance);
                    }
                }
            };
        }
    };
    Err(ProjectError::Null())
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./task.txt")?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // 关键设置：文件没有标题行
        .from_reader(file);
    for result in rdr.records() {
        let record = result?;
        // 此时 record[0] 就是第一列的数据，而不是列名
        let split = record[0].split("\t").collect::<Vec<&str>>();
        let id = split.get(0).unwrap();
        let task_no = split.get(1).unwrap();
        // https://app.shb.ltd/api/middleware/outside/moen/moenTaskMileage
        task_mileage(task_no.to_string(), id.to_string());
    }
    Ok(())
    // AZ2025091702814 AZ2025091702816 WX2025091706379
}
