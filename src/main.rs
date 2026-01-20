use std::thread;
use std::thread::sleep;
use std::time::Duration;

// fn main() {
//     // main_sync();
//     main_async();
// }

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(async {
        let _string = async_read_from_file1().await;
    });
    let handle2 = tokio::spawn(async {
        let _string = async_read_from_file2().await;
    });

    tokio::spawn(async {
        main_sync();
    });
    tokio::spawn(async {
        main_async();
    });
    let _ = tokio::join!(handle1, handle2);
}

//多线程处理
fn main_async() {
    let handle1 = thread::spawn(|| {
        let file1 = read_from_file1();
        println!("{:?}", file1);
    });
    let handle2 = thread::spawn(|| {
        let file2 = read_from_file2();
        println!("{:?}", file2);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

//同步处理
#[warn(dead_code, unused)]
fn main_sync() {
    let file1 = read_from_file1();
    println!("{:?}", file1);
    let file2 = read_from_file2();
    println!("{:?}", file2);
}
fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    String::from("hello,file1")
}
fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    String::from("hello,file2")
}
async fn async_read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    println!("Read from file1");
    String::from("hello,file1")
}
//等价于上面
// fn future_read_from_file1() -> impl Future<Output=String>  {
//     async {
//         sleep(Duration::new(4, 0));
//         println!("Read from file1");
//         String::from("hello,file1")
//     }
// }
async fn async_read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    println!("Read from file2");
    String::from("hello,file2")
}
