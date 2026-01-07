use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let s=String::from("aaa");
    let len=len(&s);
    println!("s:{} len: {}",s, len);

    let mut writer =String::from("bbb");
    change(&mut writer);
    println!("writer:{writer}");
}

fn len(s: &String) -> usize {
    s.len()
}
fn change(s: &mut String) {
    s.push_str("111");
}