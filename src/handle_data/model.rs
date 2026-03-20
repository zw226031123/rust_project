use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Request {
    pub host: String,
    pub token: String,
}
#[derive(Deserialize, Serialize)]
pub struct TaskRecordListParam {
    #[serde(rename = "taskId")]
    pub task_id: String,
    #[serde(rename = "pageNum")]
    pub page_num: i8,
    #[serde(rename = "pageSize")]
    pub page_size: i8,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "hasViewBalanceRecord")]
    pub has_view_balance_record: i8,
    #[serde(rename = "hasViewReviewRecord")]
    pub has_view_review_record: i8,
}

#[derive(Deserialize, Serialize)]
pub struct EstimatedDistanceParam {
    pub key: String,
    pub strategy: i8,
    pub origin: String,
    pub destination: String,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: String,
    #[serde(rename = "taskNo")]
    pub task_no: String,
    pub address: Address,
}
#[derive(Deserialize, Serialize)]
pub struct Address {
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

#[derive(Deserialize, Serialize)]
pub struct TaskRecord {
    pub action: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

#[derive(Deserialize, Serialize)]
pub struct AmapDrivingDistanceResponsePath {
    pub distance: String,
    pub duration: String,
    pub strategy: String,
    pub tolls: String,
    pub restriction: String,
    pub traffic_lights: String,
    pub toll_distance: String,
}
