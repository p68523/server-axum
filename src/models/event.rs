use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiEvent
{
    pub v: u32,
    pub r#type: String,
    pub ts: String,
    pub seq: u64,
    pub camera_id: String,
    pub event: EventBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBody
{
    pub event_id: String,
    pub event_type: String,
    pub severity: String,
    pub confidence: f32,
    pub bbox: Option<BBox>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BBox
{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl AiEvent
{
    pub fn demo(seq: u64) -> AiEvent
    {
        AiEvent
        {
            v: 1,
            r#type: "event".to_string(),
            ts: now_utc_iso8601(),
            seq,
            camera_id: "CAM-001".to_string(),
            event: EventBody
            {
                event_id: format!("EVT-DEMO-{}", seq),
                event_type: "intrusion".to_string(),
                severity: "high".to_string(),
                confidence: 0.92,
                bbox: Some(
                    BBox
                    {
                        x: 0.42,
                        y: 0.31,
                        w: 0.12,
                        h: 0.28,
                    }
                ),
            },
        }
    }
}

fn now_utc_iso8601() -> String
{
    Utc::now().to_rfc3339()
}
