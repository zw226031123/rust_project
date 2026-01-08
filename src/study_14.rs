use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;



fn main() {
    let secret_number = rand::rng().random_range(1..101);
    println!("Guess the number,{}", secret_number);
    loop {
        println!("猜数");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {guess}");
        let guess:i32= guess.trim().parse().expect("Please type a number!");
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
