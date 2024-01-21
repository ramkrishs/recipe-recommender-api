use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

/// Constructs and returns a router with health check endpoints.
///
/// This function adds two endpoints to the provided router:
/// - `/health/readiness`: Checks if the service is ready to handle requests.
/// - `/health/liveness`: Checks if the service is running and able to respond.
///
/// # Arguments
///
/// * `router` - A Router object to which the health check routes will be added.
///
/// # Returns
///
/// This function returns the modified Router with the new health check routes.
pub fn build_health_route(router: Router) -> Router {
    router
        .route("/health/readiness", get(readiness))
        .route("/health/liveness", get(liveness))
}

/// Handles the readiness health check request.
///
/// When this endpoint is called, it responds with a JSON object indicating
/// that the service is ready to handle requests.
///
/// # Returns
///
/// This function returns a JSON response with a "ready" key set to `true`.
async fn readiness() -> impl IntoResponse {
    Json(json!({
     "ready": true
    }))
}

/// Handles the liveness health check request.
///
/// This endpoint responds with a JSON object indicating that the service
/// is live and operational.
///
/// # Returns
///
/// This function returns a JSON response with a "live" key set to `true`.

async fn liveness() -> impl IntoResponse {
    Json(json!({
     "live": true
    }))
}
