use crate::utils::error::ApiError;
use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn method_not_allowed(request: Request<Body>, next: Next) -> Result<Response, ApiError> {
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    if response.status() == StatusCode::METHOD_NOT_ALLOWED {
        return Err(ApiError::new(
            vec![format!(
                "'{}' method is not allowed for URL '{}'",
                method, uri
            )],
            StatusCode::METHOD_NOT_ALLOWED,
        ));
    }

    Ok(response)
}
