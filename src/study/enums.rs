
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
    let home_v2 = IpAddrKindV2::V4(String::from("127.0.0.1"));
    let loopback_v2 = IpAddrKindV2::V6(String::from("::1"));
    let _a = loopback_v2;
    match home_v2 {
        IpAddrKindV2::V4(address) => {println!("{}",address);},
        IpAddrKindV2::V6(address) => {println!("{}",address);},
    }
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
