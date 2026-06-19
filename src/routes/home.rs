use axum::{Router, routing::get};
use crate::state::AppState;

async fn home() -> &'static str {
    "APT backend service is running..."
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
}