use axum::{Router, routing::{get, post, patch, delete}};
use crate::handler::recognition::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recognition/{recognition_id}", get(get_recognition_detail_hand))
        .route("/recognition", get(get_recognition_detail_hand))
        .route("/recognition/link/{link_id}", get(search_recognition_hand))
        .route("/recognition", post(create_recognition_hand))
        .route("/recognition/{recognition_id}", patch(update_recognition_hand))
        .route("/recognition/{recognition_id}", delete(delete_recognition_hand))
        .route("/recognition/{recognition_id}/approve", post(approve_recognition_hand))
        .route("/recognition/{recognition_id}/reject", post(reject_recognition_hand))

        .route("/recognition/category", get(get_all_recognition_cat_hand))
        .route("/recognition/category/{category_id}", get(get_recognition_cat_detail_hand))
        .route("/recognition/category/name/{category_name}", get(get_recognition_cat_name_hand))
        .route("/recognition/category", post(create_recognition_cat_hand))
        .route("/recognition/category/{category_id}", patch(update_recognition_cat_hand))
        .route("/recognition/category/{category_id}", delete(delete_recognition_cat_hand))
}