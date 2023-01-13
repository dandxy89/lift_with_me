use tokio::sync::broadcast::{Receiver, Sender};

use crate::model::operation::Command;

/// Interval timer
pub async fn internal_timer(tx: Sender<Command>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        tx.send(Command::Tick).expect("to be sent");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        tx.send(Command::RequestLocation).expect("to be sent");
    }
}

// Show me when each tick is called
pub async fn display_ticks(mut rx: Receiver<Command>) {
    while let Ok(cmd) = rx.recv().await {
        tracing::trace!("tick_tock from [{:?}]!", cmd);
    }
}
