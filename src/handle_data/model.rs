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
