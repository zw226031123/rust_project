fn main() {
    for i in 1..=9 {
        for j in 1..=i {
            let multi = i * j;
            print!("{i}*{j}={multi}\t")
        }
        println!()
    }
}
