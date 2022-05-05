//! Route handlers for processing requests and producing responses.
use axum::{
    extract::Path,
    http::{StatusCode, Version},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use tracing::info;

/// Gets the current status of the service
#[tracing::instrument]
pub(crate) async fn health_check() -> impl IntoResponse {
    info!("processing health check");
    StatusCode::OK
}

/// Processes incoming payloads in JSON format
#[tracing::instrument(skip(payload))]
pub(crate) async fn parse_webhook_payload(
    Path((owner, repo)): Path<(String, String)>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    info!(%owner, %repo, "parsing webhook payload");
    // For now, simply echo the payload back to the user
    let body = Json(json!({
        "message": {
            "owner": owner,
            "repo": repo,
        },
        "payload": payload
    }));

    // TODO: Remove once verified to be working
    info!(body = %body.0);

    Response::builder()
        .version(Version::HTTP_2)
        .status(StatusCode::OK)
        .body(body.into_response())
        .unwrap()
}
