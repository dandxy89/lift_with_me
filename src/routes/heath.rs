use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[allow(clippy::unused_async)]
pub async fn health() -> impl IntoResponse {
    let payload = json!({"status": "Ok"});
    (StatusCode::OK, Json(payload))
}
