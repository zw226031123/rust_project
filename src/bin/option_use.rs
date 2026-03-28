fn main() {
    let none_val: Option<i32> = None;
    let result = none_val.unwrap_or(0); // 返回 0
    println!("{}", result);
}
