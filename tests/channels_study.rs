use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn main() {
    let (ts, tr) = mpsc::channel();
    let tx = ts.clone();
    thread::spawn(move || {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        for val in vec {
            ts.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vec = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        for val in vec {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for x in tr {
        println!("{}", x);
    }
}
