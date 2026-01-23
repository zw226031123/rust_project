fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    for i in &vec {
        print!("{}\t", i);
    }
    println!();
    let result = largest(&vec);
    println!("The largest number is {}", result);
    let vec = vec!["1", "2", "3", "4", "5"];
    let result = largest(&vec);
    println!("The largest number is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
