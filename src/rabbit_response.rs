use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct RabbitResponse {
    pub items: Vec<Queue>,
}

fn current_time() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct Queue {
    #[serde(rename(serialize = "Timestamp"), default = "current_time")]
    pub timestamp: String,

    #[serde(rename(serialize = "Queue"))]
    pub name: String,

    #[serde(rename(serialize = "Memory"))]
    pub memory: u32,

    #[serde(rename(serialize = "M_Bytes"))]
    pub message_bytes: u32,

    #[serde(rename(serialize = "M_Total"))]
    pub messages: u32,

    #[serde(rename(serialize = "M_Ready"))]
    pub messages_ready: u32,

    #[serde(rename(serialize = "M_Unack"))]
    pub messages_unacknowledged: u32,

    #[serde(rename(serialize = "M_Rate"))]
    pub messages_details: Details,

    #[serde(rename(serialize = "M_Ready Rate"))]
    pub messages_ready_details: Details,

    #[serde(rename(serialize = "M_UnAck Rate"))]
    pub messages_unacknowledged_details: Details,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub rate: f64,
}
