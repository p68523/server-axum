use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};

use crate::config::Config;
use crate::state::{AppState, SharedState};

pub fn build_app(config: Config) -> Router
{
    let state = Arc::new(AppState::new(config));

    build_router(state)
}

fn build_router(state: SharedState) -> Router
{
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(crate::routes::router(state.clone()))
        .merge(crate::ws::router(state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
