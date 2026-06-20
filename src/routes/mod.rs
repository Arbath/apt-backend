pub mod home;
pub mod auth;
pub mod user;

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
    
    Router::new()
        .merge(home::routes().layer(uncors.clone()))
        .nest("/api",auth::routes()
            .layer(cors.clone())
        )
        .nest("/api/user",user::routes()
            .layer(cors.clone())
        )
        // .nest("/api",rule::routes()
        //     .layer(cors.clone())
        // )
        // .nest("/api",webhook::routes()
        //     .layer(cors.clone())
        // )

        .with_state(state)

        // Logging
        .layer(CatchPanicLayer::new()) 
        .layer(TraceLayer::new_for_http())
        
        // Security & Protocol
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        
        // Performance
        .layer(CompressionLayer::new())
}