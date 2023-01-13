use crate::{
    model::{
        elevator::add_lift,
        operation::{Command, Movement, Passenger},
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use tokio::sync::broadcast::Sender;

#[allow(clippy::module_name_repetitions)]
/// Register a new `Elevator` for Work -`POST /elevator/register/:id`
pub async fn attach_elevator(
    Path(id): Path<u8>,
    State(cmd_tx): State<Sender<Command>>,
) -> impl IntoResponse {
    tokio::spawn(async move { add_lift(id, cmd_tx).await });
    (StatusCode::CREATED, Json(json!({})))
}

#[allow(clippy::unused_async)]
/// # Panics
pub async fn passenger_request(
    State(cmd_tx): State<Sender<Command>>,
    Extension(db): Extension<AppState>,
    Json(payload): Json<Passenger>,
) -> impl IntoResponse {
    tracing::info!("Passenger event received {:?}", payload);
    if payload.from_floor == payload.to_floor {
        let payload = json!({"status": "You've arrived!'"});
        return (StatusCode::OK, Json(payload));
    }

    let active = db.lock().await;
    if active.is_empty() {
        let payload = json!({"status": "No Lifts - what are you requesting?"});
        return (StatusCode::NOT_FOUND, Json(payload));
    }

    let req_loc = payload.from_floor;
    let idle_lifts: Option<(&u8, &i16, i16)> = active
        .iter()
        .filter(|(_, (is_busy, _))| !is_busy)
        .map(|(id, (_, cf))| (id, cf, (cf - req_loc).abs()))
        .min_by(|a, b| a.2.cmp(&b.2));

    if let Some((id, cf, _)) = idle_lifts {
        // Do we need to move to Pickup floor
        if let Some(mvm) = Movement::get_direction(req_loc, *cf) {
            if cmd_tx.send(Command::Lift(*id, mvm)).is_err() {
                let payload = json!({"status": "Oh no!"});
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload));
            }
        }

        // Lets move Passenger to requested floor
        if let Some(mvm) = Movement::get_direction(payload.to_floor, payload.from_floor) {
            if cmd_tx.send(Command::Lift(*id, mvm)).is_err() {
                let payload = json!({"status": "Oh no!"});
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload));
            }
        }

        let payload = json!({"status": "Ok"});
        (StatusCode::OK, Json(payload))
    } else {
        let payload = json!({"status": "Code Path NOT_IMPLEMENTED"});
        (StatusCode::NOT_IMPLEMENTED, Json(payload))
    }
}
