use std::{sync::Arc, time::Duration};

use tokio::{sync::mpsc::Receiver, time::sleep};

const SLEEP_TIME_MILLIS: u64 = 1000 / 30;

pub async fn consumer(mut rx: Receiver<Arc<str>>) -> anyhow::Result<()> {
    while let Some(text) = rx.recv().await {
        for ch in text.chars() {
            print!("{}", ch);

            // Flush stdout immediately
            use std::io::{Write, stdout};
            stdout().flush()?;

            sleep(Duration::from_millis(SLEEP_TIME_MILLIS)).await;
        }
    }
    Ok(())
}
