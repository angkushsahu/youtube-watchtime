mod apis;
mod app;
mod entities;
mod fallback;
mod utils;

use app::{create_routes, init_cors, spa_service};
use axum::Router;
use dotenvy::dotenv;
use utils::graceful_shutdown::shutdown_signal;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = init_cors();
    let app = Router::new()
        .nest("/api", create_routes())
        .fallback_service(spa_service())
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or(String::from("8080"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("Application started 🚀....");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
