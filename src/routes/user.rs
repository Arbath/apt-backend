use axum::{Router, routing::{post, patch, delete}};
use crate::handler::user::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_user_hand))
        .route("/{user_id}/edit", patch(edit_user_hand))
        .route("/{user_id}/remove", delete(delete_user_hand))
        .route("/{user_id}/reset-password", post(reset_password_user_hand))
}