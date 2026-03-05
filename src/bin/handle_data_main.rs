use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rust_project::handle_data::http_data;
use rust_project::handle_data::model::{Request, TaskRecordListParam};
use rust_project::handle_data::shb_task;
use std::str::FromStr;
#[allow(dead_code)]
fn main1() {
    http_data::test_main();
    let _ = http_data::main();
}
fn main() {
    let request = Request {
        host: String::from("https://app.shb.ltd"),
        token: String::from("2cc11e438c2017a7361c301480266c78_41"),
    };
    let body_str = String::from(
        r#"{"page":1,"pageSize":100,"queryCreateHalf":true,"keyword":"WX2026020966970","templateId":"e418f696-c7b0-4e28-9c13-b680578dcae5","stateList":[],"isException":null,"exceptionStates":[],"exceptionNodes":[],"conditions":[],"systemConditions":[],"createUser":"","executor":"","synergyId":"","currentNodeExecutorUser":"","nodeProcessedUser":"","searchNodeId":"","whoseInfo":"my","labelQuery":{"labelIds":null,"labelExists":null},"privacy":true}"#,
    );
    let result = shb_task::search_task(&request, body_str);
    if let Ok(res) = result {
        println!("search_task:{}", res);
    };
    let param = TaskRecordListParam {
        task_id: String::from("d964fb60-057b-11f1-a4d5-00163e0b3979"),
        page_num: 1,
        page_size: 15,
        user_id: String::from("89cb970f4f38de533412e625030446fc"),
        has_view_balance_record: 1,
        has_view_review_record: 1,
    };
    let result = shb_task::task_record_list(&request, param);
    if let Ok(res) = result {
        println!("task_record_list:{}", res);
    };

    // let estimated_distance= estimated_distance(Decimal::from_str("106.843702").unwrap(), Decimal::from_str("27.206202").unwrap(), Decimal::from_str("106.616778").unwrap(), Decimal::from_str("26.610012").unwrap());
    // println!("{:?}",estimated_distance)
    let _ = shb_task::main();
    let _ = http_data::main();
}
