use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc::Receiver;

pub struct MyReceiverStream<T> {
    rx: Receiver<T>,
}

impl<T> MyReceiverStream<T> {
    pub fn new(rx: Receiver<T>) -> Self {
        MyReceiverStream { rx }
    }
}

impl<T> Stream for MyReceiverStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // delegate to the Receiver's poll_recv (tokio exposes this)
        Pin::new(&mut self.rx).poll_recv(cx)
    }
}

fn main() {
    
}
