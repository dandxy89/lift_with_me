use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use lift_with_me::{
    routes::{
        heath::health,
        lift::{attach_elevator, passenger_request},
        status::{get_current_status_all, get_lift_status},
    },
    scheduler::task_scheduler,
    ticker::internal_ticking,
    AppState,
};
use tokio::sync::Mutex;

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

    // Broadcasting ticks - fake moving through time...
    let (tx, _) = tokio::sync::broadcast::channel(32);

    // Task Scheduler
    let tx_new = tx.clone();
    let app_state1 = app_state.clone();
    tokio::spawn(async move { task_scheduler(tx_new.subscribe(), app_state1).await });

    // Start the Clock
    let tx_new = tx.clone();
    let clock = tokio::spawn(async move { internal_ticking(tx_new).await });
    // let rx2 = tx.subscribe();
    // let _listen = tokio::spawn(async move { display_ticks(rx2).await });

    // Define a Router
    let app = Router::new()
        .route("/status", get(health))
        .route("/elevator/register/:id", post(attach_elevator))
        .route("/elevator/request", post(passenger_request))
        .route("/elevator/status", get(get_current_status_all))
        .route("/elevator/status/:id", post(get_lift_status))
        .layer(Extension(app_state))
        .with_state(tx);

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    clock.abort();
}
