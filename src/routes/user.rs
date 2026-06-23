use axum::{Router, routing::{post, patch, delete}};
use crate::handler::user::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user", post(add_user_hand))
        .route("/user/{user_id}", patch(edit_user_hand))
        .route("/user/{user_id}", delete(delete_user_hand))
        .route("/user/{user_id}/reset-password", post(reset_password_user_hand))
}