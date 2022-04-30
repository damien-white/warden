use axum::{
    extract::Path,
    http::{StatusCode, Version},
    response::{IntoResponse, Response},
    routing::{get, post, Router},
    Json,
};
use serde_json::{json, Value};

pub fn app() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/webhooks/:owner/:repo", post(parse_webhook_payload))
}

/// Gets the current status of the service
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

/// Processes incoming payloads in JSON format
async fn parse_webhook_payload(
    Path((owner, repo)): Path<(String, String)>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    // For now, simply echo the payload back to the user
    let body = Json(json!({
        "message": {
            "owner": owner,
            "repo": repo,
        },
        "payload": payload
    }));

    Response::builder()
        .version(Version::HTTP_2)
        .status(StatusCode::OK)
        .body(body.into_response())
        .unwrap()
}
