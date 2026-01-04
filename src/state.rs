use std::sync::Arc;
use tokio::sync::broadcast;

use crate::config::Config;
use crate::models::event::AiEvent;

#[derive(Clone)]
pub struct AppState
{
    pub config: Config,
    pub event_tx: broadcast::Sender<AiEvent>,
}

impl AppState
{
    pub fn new(config: Config) -> AppState
    {
        let (event_tx, _rx) = broadcast::channel::<AiEvent>(1024);

        AppState { config, event_tx }
    }
}

pub type SharedState = Arc<AppState>;
