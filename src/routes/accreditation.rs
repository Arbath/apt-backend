use axum::{Router, routing::{get, post, patch, delete}};
use crate::handler::accreditation::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/accreditation/{accreditation_id}", get(get_accreditation_detail))
        .route("/accreditation",get(get_all_accreditation))
        .route("/accreditation",post(create_accreditation))
        .route("/accreditation/{accreditation_id}", patch(update_accreditation))
        .route("/accreditation/{accreditation_id}", delete(delete_accreditation))
        
        .route("/accreditation/indicator/{indicator_id}", get(get_indicator_detail))
        .route("/accreditation/indicator",get(search_indicator))
        .route("/accreditation/indicator",post(create_indicator))
        .route("/accreditation/indicator/{indicator_id}", patch(update_indicator))
        .route("/accreditation/indicator/{indicator_id}", delete(delete_indicator))
        
        .route("/accreditation/indicator/rule/{rule_id}", get(get_calculation_detail))
        .route("/accreditation/indicator/rule",get(search_calculation))
        .route("/accreditation/indicator/rule",post(create_calculation))
        .route("/accreditation/indicator/rule/{rule_id}", patch(update_calculation))
        .route("/accreditation/indicator/rule/{rule_id}", delete(delete_calculation))
        
        .route("/accreditation/indicator/evaluation/{evaluation_id}", get(get_evaluation_detail))
        .route("/accreditation/indicator/evaluation",get(search_evaluation))
        .route("/accreditation/indicator/evaluation",post(create_evaluation))
        .route("/accreditation/indicator/evaluation/{evaluation_id}", patch(update_evaluation))
        .route("/accreditation/indicator/evaluation/{evaluation_id}", delete(delete_evaluation))
}