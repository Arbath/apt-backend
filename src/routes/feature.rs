use axum::{Router, routing::{get, post, patch, delete}};
use crate::handler::feature::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/link/{link_id}", get(get_link_by_id_hand))
        .route("/link/slug/{slug}", get(get_link_by_slug_hand))
        .route("/link", get(get_links_by_institute_hand))
        .route("/link", post(create_link_hand))
        .route("/link/{link_id}", patch(update_link_hand))
        .route("/link/{link_id}", delete(delete_link_hand))

        .route("/log",get(search_logs))
        .route("/log/{log_id}", get(get_log_detail))
        .route("/log/{log_id}",delete(delete_log))
        .route("/log-older-than/{days}",delete(delete_logs))
}