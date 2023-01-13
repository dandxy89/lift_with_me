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
                        tracing::info!("Updating status for [{}] on floor [{}]", loc_stat.id, loc_stat.floor);
                        {
                            app_state
                                .lock()
                                .unwrap()
                                .entry(loc_stat.id)
                                .and_modify(|e| {
                                    *e = (loc_stat.is_busy, loc_stat.floor);
                                });
                        }
                    }
                    Command::Register(loc_stat) => {
                        {
                            app_state
                                .lock()
                                .unwrap()
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
