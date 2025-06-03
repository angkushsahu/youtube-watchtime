use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    success: bool,
    errors: Vec<String>,
    status_code: u16,
}

impl ApiError {
    pub fn new(errors: Vec<String>, status_code: StatusCode) -> Self {
        Self {
            success: false,
            errors,
            status_code: status_code.as_u16(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(self);
        (status_code, body).into_response()
    }
}
