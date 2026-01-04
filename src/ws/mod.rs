use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::state::SharedState;

pub mod handler;
pub mod protocol;

pub fn router(state: SharedState) -> Router
{
    let static_dir = state.config.static_dir.clone();

    Router::new()
        .route("/ws/events", get(handler::ws_events))
        .nest_service("/static", ServeDir::new(static_dir).precompressed_gzip())
        .with_state(state)
}
