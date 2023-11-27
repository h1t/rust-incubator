use std::{
    future::{self, Future},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Wake},
    time::{Duration, Instant},
};

use pin_project::pin_project;

#[pin_project]
struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,

    started_at: Option<Instant>,
}

impl<Fut> MeasurableFuture<Fut> {
    fn new(future: Fut) -> Self {
        Self {
            inner_future: future,
            started_at: None,
        }
    }
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let start = this.started_at.get_or_insert_with(Instant::now);

        match this.inner_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => Poll::Ready((output, start.elapsed())),
        }
    }
}

struct NoopWaker;

impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

pub fn check() {
    let mut future = MeasurableFuture::new(future::ready(5));
    // pin::pin!(future);
    let mut future = Pin::new(&mut future);

    let waker = Arc::new(NoopWaker).into();
    let mut cx = Context::from_waker(&waker);

    if let Poll::Ready((output, duration)) = future.as_mut().poll(&mut cx) {
        println!("{output} {}", duration.as_micros());
    }
}
