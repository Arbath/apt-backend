use axum::{Router, routing::{get, post, patch, delete}};
use crate::handler::institute::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/institute", get(search_institute_hand))
        .route("/institute/{institute_id}/programs", get(get_all_institute_study_programs_hand))
        .route("/institute/{institute_id}", get(get_one_institute_hand))
        .route("/institute", post(add_institute_hand))
        .route("/institute/{institute_id}", patch(edit_institute_hand))
        .route("/institute/{institute_id}", delete(delete_institute_hand))

        .route("/study-program", get(search_study_program_hand))
        .route("/study-program/{program_id}", get(get_one_study_program_hand))
        .route("/study-program", post(add_study_program_hand))
        .route("/study-program/{program_id}", patch(edit_study_program_hand))
        .route("/study-program/{program_id}", delete(delete_study_program_hand))
}