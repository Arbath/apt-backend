use axum::{Router, routing::{get, post, patch, delete}};
use crate::handler::lecturer::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/lecturer/{lecturer_id}", get(get_lecturer_detail_hand))
        .route("/lecturer/nip/{lecturer_nip}", get(get_lecturer_nip_hand))
        .route("/lecturer", get(search_lecturer_hand))
        .route("/lecturer", post(add_lecturer_hand))
        .route("/lecturer/{lecturer_id}", patch(edit_lecturer_hand))
        .route("/lecturer/{lecturer_id}", delete(delete_lecturer_hand))
        .route("/lecturer/{lecturer_id}/approve", post(approve_lecturer_hand))
        .route("/lecturer/{lecturer_id}/reject", post(reject_lecturer_hand))
}