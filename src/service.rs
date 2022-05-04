mod handlers;

use axum::routing::{get, post};
use axum::Router;
use tower_http::trace::TraceLayer;

use handlers::{health_check, parse_webhook_payload};

pub fn app() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/webhooks/:owner/:repo", post(parse_webhook_payload))
        .layer(TraceLayer::new_for_http())
}
