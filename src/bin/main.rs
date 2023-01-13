use std::net::SocketAddr;

use axum::{routing::get, Router};
use lift_with_me::{
    model::{
        elevator::add_lift,
        operation::{Command, LocationStatus},
    },
    routes::heath::health,
    ticker::{display_ticks, internal_timer},
};
use tokio::sync::broadcast::{self, Receiver, Sender};

async fn task_scheduler(
    n_elevators: u8,
    mut status: Receiver<LocationStatus>,
    _elevator_job_tx: Sender<Command>,
) {
    let mut avaliable_for_work = vec![(true, 0); n_elevators as usize];
    loop {
        tokio::select! {
            Ok(loc_stat) = status.recv() => {
                tracing::info!("Updating status for [{}] on floor [{}]", loc_stat.id, loc_stat.floor);
                avaliable_for_work[loc_stat.id as usize] = (loc_stat.is_busy, loc_stat.floor);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Logging
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let n_elevators = 2;

    // Broadcasting ticks - fake moving through time...
    let (tx, _rx) = broadcast::channel(32);
    let (tx_s, rx_s) = broadcast::channel::<LocationStatus>(32);

    // Task Scheduler
    let tx_new = tx.clone();
    tokio::spawn(async move { task_scheduler(n_elevators, rx_s, tx_new).await });

    // Create some elevators
    for elevator_id in 0..n_elevators {
        let rx_new = tx.subscribe();
        let tx_new = tx_s.clone();
        tokio::spawn(async move { add_lift(elevator_id, rx_new, tx_new).await });
    }

    // Start the Clock
    let rx2 = tx.subscribe();
    let _clock = tokio::spawn(async move { internal_timer(tx).await });
    let _listen = tokio::spawn(async move { display_ticks(rx2).await });

    // build our application with a route
    let app = Router::new()
        // `GET /status
        .route("/status", get(health));
    // TODO
    // Get the Current Status of an Elevator
    //  `GET /elevator/status/{ID}
    //  `GET /elevator/status
    // Add requests to Elevator Queues
    //  `POST /request
    //  `POST /request/{ID}
    // Register a new Elevator for Work
    //  `POST /elevator/register

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
