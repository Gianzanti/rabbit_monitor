use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RabbitResponse {
    pub items: Vec<Queue>,
}

fn current_time() -> DateTime<Utc> {
    // chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    chrono::Utc::now()
}

fn zero_stats() -> Stats {
    Stats {
        // publish: 0,
        publish_details: Details { rate: 0.0 },
        // ack: 0,
        ack_details: Details { rate: 0.0 },
        // deliver_get: 0,
        deliver_get_details: Details { rate: 0.0 },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    #[serde(rename(serialize = "Timestamp"), default = "current_time")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename(serialize = "Queue"))]
    pub name: String,

    // // #[serde(rename(serialize = "Memory"))]
    // #[serde(skip)]
    // pub memory: u32,

    // // #[serde(rename(serialize = "M_Bytes"))]
    // #[serde(skip)]
    // pub message_bytes: u32,
    #[serde(rename(serialize = "M_Total"))]
    pub messages: u32,

    #[serde(rename(serialize = "M_Unack"))]
    pub messages_unacknowledged: u32,

    #[serde(rename(serialize = "M_Ready"))]
    pub messages_ready: u32,

    // // #[serde(rename(serialize = "M_Rate"))]
    // #[serde(skip)]
    // pub messages_details: Details,

    // // #[serde(rename(serialize = "M_Ready Rate"))]
    // #[serde(skip)]
    // pub messages_ready_details: Details,

    // #[serde(rename(serialize = "M_UnAck Rate"))]
    // pub messages_unacknowledged_details: Details,
    #[serde(rename(serialize = "M_Ready Rate"), default = "zero_stats")]
    pub message_stats: Stats,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Details {
    pub rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Stats {
    // pub publish: u32,
    pub publish_details: Details,
    // pub ack: u32,
    pub ack_details: Details,
    // pub deliver_get: u32,
    pub deliver_get_details: Details,
}
