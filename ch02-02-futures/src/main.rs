use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::task::JoinHandle;

struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    let x = String::from("asd");

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    let counter_one = CounterFuture { count: 0 };
    let counter_two = CounterFuture { count: 0 };
    let handle_one = tokio::task::spawn(async move { counter_one.await });
    let handle_two  = tokio::task::spawn(async move { counter_two.await });
    let _ = tokio::join!(handle_one, handle_two);
}
