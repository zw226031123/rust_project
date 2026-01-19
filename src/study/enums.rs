
fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let _kind = loopback.kind;
    println!("{}",loopback.address);
    let _home_v2 = IpAddrKindV2::V4(String::from("127.0.0.1"));
    let _a = _home_v2;
    println!("{:p}", &_a);
    let loopback_v2 = IpAddrKindV2::V6(String::from("::1"));
    let _a = loopback_v2;
    println!("{:p}", &_a);
    // Option::Some("Hello").map(|i| i.len()).or_else(0);
}
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKindV2 {
    V4(String),
    V6(String),
}
struct IpAddr {
    #[warn(dead_code)]
    kind: IpAddrKind,
    address: String,
}
