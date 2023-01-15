use crate::{
    model::operation::{add_lift, Command, Movement, Passenger},
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
    Json(request): Json<Passenger>,
) -> impl IntoResponse {
    tracing::info!("Passenger event received {:?}", request);
    if request.from_floor == request.to_floor {
        let payload = json!({"status": "No action required"});
        return (StatusCode::ACCEPTED, Json(payload));
    }

    let active = db.lock().await;
    if active.is_empty() {
        let payload = json!({"status": "No Lifts - what are you requesting?"});
        return (StatusCode::NOT_FOUND, Json(payload));
    }

    let req_loc = request.from_floor;
    let nearest_lifts: Vec<(u8, bool, i16)> = active
        .iter()
        .map(|(id, loc_stat)| (*id, loc_stat.is_busy, (loc_stat.floor - req_loc).abs()))
        .collect();
    let idle_lifts: Option<&(u8, bool, i16)> = nearest_lifts
        .iter()
        .filter(|(_, is_busy, _)| !is_busy)
        .min_by(|a, b| a.2.cmp(&b.2));

    if let Some((id, _, _)) = idle_lifts {
        tracing::info!("Found Idle Lift to assign work to Id=[{id}]");
        if let Some(mvm) = Movement::get_direction(req_loc, request.from_floor) {
            if cmd_tx.send(Command::Lift(*id, mvm)).is_err() {
                let payload = json!({"status": "Unable to notify "});
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload));
            }
        }

        let payload = json!({"status": "Ok"});
        return (StatusCode::OK, Json(payload));
    }

    if let Some((id, _, _)) = nearest_lifts.iter().min_by(|a, b| a.2.cmp(&b.2)) {
        tracing::info!("Found Nearest Lift to assign work to Id=[{id}]");
        if let Some(mvm) = Movement::get_direction(req_loc, request.from_floor) {
            if cmd_tx.send(Command::Lift(*id, mvm)).is_err() {
                let payload = json!({"status": "Oh no!"});
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload));
            }
        }

        let payload = json!({"status": "Ok"});
        return (StatusCode::OK, Json(payload));
    }

    let payload = json!({"status": "Oh no!"});
    (StatusCode::INTERNAL_SERVER_ERROR, Json(payload))
}
