use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsEnvelope<T>
{
    pub v: u32,
    pub r#type: String,
    pub ts: String,
    pub seq: u64,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscribe
{
    pub client_id: String,
    pub camera_ids: Vec<String>,
    pub event_types: Vec<String>,
    pub min_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack
{
    pub event_id: String,
    pub event_seq: u64,
}
