use base64::Engine;
use base64::prelude::BASE64_STANDARD;

#[actix_rt::main]
async fn main() {
    trpl::run(async {
        let client = awc::Client::default();
        let authorization = base64("publink", "rxe3N@9%");
        let response = client
            .get("http://flink-console.linker.ltd/jobs/overview")
            .insert_header(("authorization", authorization))
            .send()
            .await
            .unwrap();
        println!("{:#?}", response);
    });
}
fn base64(username: &str, password: &str) -> String {
    let authorization = format!(r#"{}:{}"#, username, password);
    BASE64_STANDARD.encode(authorization.as_bytes())
}
