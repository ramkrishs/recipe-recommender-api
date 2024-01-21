// Import necessary modules from axum framework, serde for serialization, etc.
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

/// Represents a welcome response with a message.
///
/// This struct is serialized into JSON format when sent as a response.
#[derive(Serialize)]
struct WelcomeResponse {
    message: String,
}

/// Adds welcome routes to the provided router.
///
/// This function registers two endpoints:
/// - `/`: A welcome message for all visitors.
/// - `/:name`: A personalized welcome message using the provided name.
///
/// # Arguments
///
/// * `route` - A Router object to which the welcome routes will be added.
///
/// # Returns
///
/// This function returns the modified Router with the new welcome routes.
pub fn build_welcome_route(route: Router) -> Router {
    route
        .route("/", get(welcome_msg))
        .route("/:name", get(welcome_msg_with_username))
}

/// Handles requests to the root endpoint.
///
/// Responds with a generic welcome message to the recipe recommender service.
///
/// # Returns
///
/// Returns an `OK` status code along with a JSON-formatted welcome message.
async fn welcome_msg() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(WelcomeResponse {
            message: "Hello, welcome to recipe-recommender-service".into(),
        }),
    )
}

/// Handles requests to the personalized welcome endpoint.
///
/// This function takes a `name` parameter from the path and responds with a
/// personalized welcome message.
///
/// # Arguments
///
/// * `name` - The name of the user, extracted from the URL path.
///
/// # Returns
///
/// Returns an `OK` status code along with a JSON-formatted personalized welcome message.
async fn welcome_msg_with_username(Path(name): Path<String>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(WelcomeResponse {
            message: format!("Hello, {}! Welcome to recipe-recommender-service", name).into(),
        }),
    )
}
