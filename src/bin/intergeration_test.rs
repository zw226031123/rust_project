use rust_project::models::enums::YesNo;
use rust_project::models::structs::HousePrice;

fn main() {
    let _price = HousePrice {
        price: 100,
        area: String::from("zz"),
        bed_rooms: 3,
        main_road: YesNo::YES,
    };
}

#[test]
fn test_closure() {
    let closure = |x| x;
    let _x = closure(String::from("hello"));
    // let string = closure(1);

    let mut vec = vec![1, 2, 3];
    println!("before {:?}", vec);
    let mut f = || vec.push(8);
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
    let mut add_suffix = || s.push_str(", world!");
    // println!("{}",s);
    add_suffix();
}
#[test]
fn test_closure_3() {
    let mut s = String::from("hello");
    let add_suffix = |s: &mut String| s.push_str(", world!");
    println!("{}", s);
    add_suffix(&mut s);
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
#[allow(unused)]
async fn hello_world() {
    println!("hello, world!");
}
use futures::executor::block_on;
#[allow(unused)]
fn future_test() {
    let future = hello_world();
    block_on(future);
}
