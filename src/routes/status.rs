#![allow(clippy::unused_async)]

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{model::operation::LocationStatus, AppState};

/// # Panics
pub async fn get_current_status_all(Extension(db): Extension<AppState>) -> impl IntoResponse {
    let payload: Vec<LocationStatus> = db
        .lock()
        .await
        .iter()
        .map(|(k, (b, f))| LocationStatus::new(*k, *b, *f))
        .collect();
    (StatusCode::OK, Json(json!(payload)))
}

#[allow(clippy::module_name_repetitions)]
/// # Panics
pub async fn get_lift_status(
    Path(id): Path<u8>,
    Extension(db): Extension<AppState>,
) -> impl IntoResponse {
    if let Some((is_busy, current_floor)) = db.lock().await.get(&id) {
        let payload = json!({"current_floor": current_floor, "is_busy": is_busy});
        (StatusCode::OK, Json(payload))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({})))
    }
}
