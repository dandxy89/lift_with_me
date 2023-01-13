use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use lift_with_me::{
    model::elevator::{add_lift, create_lift},
    routes::heath::health,
    scheduler::task_scheduler,
    ticker::{display_ticks, internal_timer},
    AppState,
};

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

    // "App State"
    let app_state: AppState = Arc::new(Mutex::new(HashMap::new()));
    let n_elevators = 1;

    // Broadcasting ticks - fake moving through time...
    let (tx, _rx) = tokio::sync::broadcast::channel(32);

    // Task Scheduler
    let tx_new = tx.clone();
    let app_state1 = app_state.clone();
    tokio::spawn(async move { task_scheduler(tx_new.subscribe(), app_state1).await });

    // Create some elevators
    for elevator_id in 0..n_elevators {
        let tx_new = tx.clone();
        tokio::spawn(async move { add_lift(elevator_id, tx_new).await });
    }

    // Start the Clock
    let rx2 = tx.subscribe();
    let tx_new = tx.clone();
    let _clock = tokio::spawn(async move { internal_timer(tx_new).await });
    let _listen = tokio::spawn(async move { display_ticks(rx2).await });

    // build our application with a route
    let app = Router::new()
        // `GET /status
        .route("/status", get(health))
        // `POST /elevator/register/:id
        .route("/elevator/register/:id", post(create_lift))
        // Command Transmitter
        .with_state(tx.clone());

    // TODO
    // Add requests to Elevator Queues
    //  `POST /request
    //  `POST /request/{ID}

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
