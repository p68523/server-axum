// app.rs

use std::sync::Arc;
use std::time::Duration;

use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::Router;

use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::state::{AppState, SharedState};

pub fn build_app(config: Config) -> Router
{
    let state = Arc::new(AppState::new(config));

    build_router(state)
}

fn build_router(state: SharedState) -> Router
{
    let cors = build_cors_layer(&state.config);
    let timeout_layer = TimeoutLayer::with_status_code(
        StatusCode::REQUEST_TIMEOUT,
        Duration::from_millis(state.config.http_request_timeout_ms),
    );

    Router::new()
        .merge(crate::routes::router(state.clone()))
        .merge(crate::ws::router(state.clone()))
        .layer(DefaultBodyLimit::max(state.config.http_max_body_bytes))
        .layer(timeout_layer)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

fn build_cors_layer(config: &Config) -> CorsLayer
{
    let allow_origin = build_allow_origin(config);

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true)
}

fn build_allow_origin(config: &Config) -> AllowOrigin
{
    let mut list: Vec<HeaderValue> = Vec::new();

    for origin in &config.cors_allowed_origins
    {
        match HeaderValue::from_str(origin.as_str())
        {
            Ok(v) =>
            {
                list.push(v);
            }
            Err(_) =>
            {
                tracing::warn!("invalid CORS origin ignored: {}", origin);
            }
        }
    }

    AllowOrigin::list(list)
}