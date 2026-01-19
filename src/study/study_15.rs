fn main1() {
    let mut num: i32 = 1;
    let x = &num;
    let z = *x;
    num += z;
    println!("num: {},z: {}", num, z);

    let mut strs = vec![String::from("A"), String::from("B")];
    let first = get_first(&strs);
    // strs.push(String::from("C"));
    if first.len() > 0 {
        strs.push(String::from("C"));
    }
}
fn get_first(v: &Vec<String>) -> &str {
    &v[0]
}
fn main() {
    if false {
        main1()
    }

    let mut v = vec![1, 2, 3];
    let _num = &v[2];
    v.push(1);
    // print!("{}", *num);
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(2);

    let v = vec![1, 2, 3];
    let third = &v[2];
    println!("{}", third);
    let third = v.get(2);
    match third {
        Some(x) => println!("{}", x),
        None => println!("not found"),
    };
}
