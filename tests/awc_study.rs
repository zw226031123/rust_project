#[test]
fn test(){
    let client = awc::Client::default();

    client.get("http://httpbin.org/ip").send();
}

