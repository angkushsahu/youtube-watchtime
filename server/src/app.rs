use axum::{
    Router,
    http::{self, HeaderValue, Method},
    middleware::from_fn,
    routing::{get, post},
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::{
    apis::{health::health_check, playlist_details::playlist_details},
    fallback::{method_not_allowed::method_not_allowed, not_found::not_found},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/playlist-details", post(playlist_details))
        .layer(from_fn(method_not_allowed))
        .fallback(not_found)
}

pub fn init_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::GET])
}

pub fn spa_service() -> ServeDir<ServeFile> {
    let spa_root = match std::env::var("ENVIRONMENT") {
        Ok(val) if val == "production" => String::from("/app/web"),
        _ => String::from("../web/dist"),
    };

    ServeDir::new(&spa_root).fallback(ServeFile::new(format!("{}/index.html", &spa_root)))
}
