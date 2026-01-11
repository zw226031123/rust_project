use rust_project::models::enums::YesNo;
use rust_project::models::structs::HousePrice;
use std::thread;

#[test]
fn test() {
    let price = HousePrice {
        price: 100,
        area: String::from("zz"),
        bed_rooms: 3,
        main_road: YesNo::YES,
    };
}

#[test]
fn test_closure() {
    let closure = |x| x;
    let x = closure(String::from("hello"));
    // let string = closure(1);

    let mut vec = vec![1, 2, 3];
    println!("before {:?}", vec);
    let mut  f=|| {vec.push(8)};
    // println!("before {:?}", vec);
    f();
    println!("after {:?}", vec);

    let vec = vec![1, 2, 3];
    println!("{:?}", vec);
    thread::spawn(move || {
        println!("Form {:?}", vec);
    })
    .join()
    .unwrap();
}
#[test]
fn test_closure_2() {
    let mut s = String::from("hello");
    let mut  add_suffix = || s.push_str(", world!");
    // println!("{}",s);
    add_suffix();
}
#[test]
fn test_closure_3() {
    let mut s = String::from("hello");
    let add_suffix = |s:&mut String| s.push_str(", world!");
    println!("{}",s);
    add_suffix(&mut s);
}