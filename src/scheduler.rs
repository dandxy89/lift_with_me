use crate::model::operation::Command;
use crate::AppState;

use tokio::sync::broadcast::Receiver;

#[allow(clippy::module_name_repetitions)]
/// # Panics
pub async fn task_scheduler(mut lift_job_rx: Receiver<Command>, app_state: AppState) {
    loop {
        tokio::select! {
            Ok(cmd) = lift_job_rx.recv() => {
                match cmd {
                    Command::SendLocation(loc_stat) => {
                        tracing::debug!("Updating status of Lift=[{}]", loc_stat.id);
                        {
                            app_state
                                .lock()
                                .await
                                .entry(loc_stat.id)
                                .and_modify(|e| {
                                    *e = (loc_stat.is_busy, loc_stat.floor);
                                });
                        }
                    }
                    Command::Register(loc_stat) => {
                        tracing::info!("Registering Lift=[{}]", loc_stat.id);
                        {
                            app_state
                                .lock()
                                .await
                                .entry(loc_stat.id)
                                .or_insert_with(|| (loc_stat.is_busy, loc_stat.floor));
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}
