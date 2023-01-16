use tokio::sync::broadcast::Sender;

use crate::model::operation::Command;

pub async fn internal_ticking(tx: Sender<Command>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        tx.send(Command::Tick).expect("to be sent");
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        tx.send(Command::RequestLocation).expect("to be sent");
    }
}

// pub async fn display_ticks(mut rx: Receiver<Command>) {
//     while let Ok(cmd) = rx.recv().await {
//         tracing::trace!("tick_tock from [{:?}]!", cmd);
//     }
// }
