pub mod home;
pub mod auth;
pub mod user;
pub mod institute;
pub mod lecturer;
pub mod recognition;
pub mod feature;

use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    catch_panic::CatchPanicLayer
};
use http::{status::StatusCode};
use core::time::Duration;
use crate::{state::AppState};

pub fn create_app(state: AppState) -> Router {
    let cors = state.app_config.cors.clone();
    let uncors = CorsLayer::permissive();

    // cors endpoint
    let api_routes = Router::new()
        .merge(auth::routes())
        .merge(user::routes())
        .merge(institute::routes())
        .merge(lecturer::routes())
        .merge(recognition::routes())
        .merge(feature::routes())
        .layer(cors);

    // uncors endpoint
    let home_routes = home::routes()
        .layer(uncors);
    
    Router::new()
        .merge(home_routes)
        .nest("/api", api_routes)
        .with_state(state)

        // Logging
        .layer(CatchPanicLayer::new()) 
        .layer(TraceLayer::new_for_http())
        
        // Security & Protocol
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        
        // Performance
        .layer(CompressionLayer::new())
}