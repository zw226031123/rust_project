use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct ReadFileFuture {}
impl Future for ReadFileFuture {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("tokio stop poll");
        cx.waker().wake_by_ref();
        Poll::Ready(String::from("tokio file 1"))
    }
}
struct AsyncTimer {
    expiration_time: Instant,
}
impl Future for AsyncTimer {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("tokio timer expired");
            Poll::Ready(String::from("Future timer completed"))
        } else {
            println!("Future timer not yet");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            thread::spawn(move || {
                let now = Instant::now();
                if expiration_time < now {
                    thread::sleep(Duration::new(5, 0));
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}
#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(async {
        let future = ReadFileFuture {};
        let string = future.await;
        println!("{}", string);
    });

    let handle2 = tokio::spawn(async {
        let string = read_file2().await;
        println!("{}", string);
    });

    let handle3 = tokio::spawn(async {
        let timer = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000),
        }
        .await;
        println!("{}", timer);
    });

    let _ = tokio::join!(handle1, handle2, handle3);
}
fn read_file2() -> impl Future<Output = String> {
    async {
        // tokio::time::sleep(Duration::new(2, 0)).await;
        // yield_now().await;
        sleep(Duration::new(2, 0));
        println!("tokio file2");
        String::from("hello ,file2")
    }
}
