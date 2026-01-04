use axum::Router;

use crate::state::SharedState;

pub mod health;
pub mod hello;

pub fn router(state: SharedState) -> Router
{
    Router::new()
        .merge(health::router())
        .merge(hello::router(state))
}
