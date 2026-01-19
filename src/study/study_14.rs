use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
#[warn(unused,dead_code)]
fn main1() {
    let secret_number = rand::rng().random_range(1..101);
    println!("Guess the number,{}", secret_number);
    loop {
        println!("猜数");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {guess}");
        let guess: i32 = guess.trim().parse().expect("Please type a number!");
        // match guess {
        //     Ok(num) => num,
        //     Err(_) =>continue
        // };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[warn(unused,dead_code)]
fn main2() {
    let _c: char = 'A';
    let _number: i32 = 10;
    let tup = (1, 2.3);
    let (_a, _b) = tup;

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for element in arr {
        println!("{}", element);
    }
}

fn main() {
    let _b = Box::new(0);
    let m1 = String::from("hello");
    println!("m1: {}", m1);
    println!("m1: {}", &m1);
    println!("m1: {:p}", &m1);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let num = &mut v[5];
    *num += 1;
    println!("num: {}", *num);
    println!("v: {:?}", v);

    main1();
    main2();
}
