use std::sync::Arc;

use tokio::sync::mpsc::channel;

use crate::{consumer::consumer, producer::producer};

mod consumer;
mod producer;

#[tokio::main]
async fn main() {
    let input = include_str!("input.txt");
    let (tx, rx) = channel::<Arc<str>>(128);
    let consumer_handle = tokio::spawn(consumer(rx));
    let producer_handle = tokio::spawn(producer(input.to_owned(), tx));
    let (producer_res, consumer_res) = tokio::join!(producer_handle, consumer_handle);
    if let Err(err) = producer_res {
        eprintln!("encountered producer error: {err}");
    }
    if let Err(err) = consumer_res {
        eprintln!("encountered consumer error: {err}");
    }
}
