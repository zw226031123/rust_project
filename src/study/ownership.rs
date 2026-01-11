use std::rc::Rc;
fn main() {
    // let value= return_string();
    //  println!("{}", value);
}

// fn return_string()->&String{
//     let s = String::from("hello");
//     &s
// }

fn return_string_v3() -> Rc<String> {
    let s = Rc::new(String::from("hello"));
    Rc::clone(&s)
}
fn return_string_v2() -> &'static str {
    "hello"
}
fn return_string_v1() -> String {
    let s = String::from("hello");
    s
}
