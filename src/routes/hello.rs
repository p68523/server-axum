use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::state::SharedState;

pub fn router(state: SharedState) -> Router
{
    Router::new()
        .route("/hello/{name}", get(hello))
        .with_state(state)
}

async fn hello(
    State(_state): State<SharedState>,
    Path(name): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse
{
    let ua: Option<&str> = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok());

    match ua
    {
        Some(s) =>
        {
            (StatusCode::OK, format!("Hello {} from {}", name, s))
        }
        None =>
        {
            (StatusCode::OK, format!("Hello {}", name))
        }
    }
}
