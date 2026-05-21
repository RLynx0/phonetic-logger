use std::sync::Arc;

use phonetisaurus_g2p::PhonetizationResult;
use tokio::sync::mpsc::channel;

use crate::{consumer::consumer, producer::producer};

const PHONETISAURUS_MODEL: &[u8] = include_bytes!("../models/nb_e_written.fst");

mod consumer;
mod producer;

#[tokio::main]
async fn main() {
    let input = include_str!("input.txt");
    let (tx, rx) = channel::<Packet>(128);
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

enum Packet {
    Phonetic(PhonetizationResult),
    Chars(Arc<str>),
}
