use crate::service::{health::build_health_route, home::build_welcome_route};
use axum::Router;

pub mod service;

#[tokio::main]
async fn main() {
    let service = Router::new();
    let service = build_health_route(service);
    let service = build_welcome_route(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // Start Server
    axum::serve(listener, service.into_make_service())
        .await
        .unwrap();
}
