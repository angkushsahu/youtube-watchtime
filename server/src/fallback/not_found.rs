use crate::utils::error::ApiError;
use axum::http::StatusCode;

pub async fn not_found() -> ApiError {
    ApiError::new(
        vec![String::from("The requested resource was not found")],
        StatusCode::NOT_FOUND,
    )
}
