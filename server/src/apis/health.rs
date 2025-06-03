use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    success: bool,
    message: String,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        success: true,
        message: String::from("Healthy 💪"),
    })
}
