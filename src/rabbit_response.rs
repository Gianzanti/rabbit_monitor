use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RabbitResponse {
    pub items: Vec<Queue>,
}

fn current_time() -> DateTime<Utc> {
    chrono::Utc::now()
}

fn zero_stats() -> Stats {
    Stats {
        publish_details: Details { rate: 0.0 },
        ack_details: Details { rate: 0.0 },
        deliver_get_details: Details { rate: 0.0 },
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Queue {
    #[serde(rename(serialize = "Timestamp"), default = "current_time")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename(serialize = "Queue"), default)]
    pub name: String,

    #[serde(rename(serialize = "M_Total"), default)]
    pub messages: u32,

    #[serde(rename(serialize = "M_Unack"), default)]
    pub messages_unacknowledged: u32,

    #[serde(rename(serialize = "M_Ready"), default)]
    pub messages_ready: u32,

    #[serde(rename(serialize = "M_Ready Rate"), default = "zero_stats")]
    pub message_stats: Stats,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Details {
    pub rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Stats {
    pub publish_details: Details,
    pub ack_details: Details,
    pub deliver_get_details: Details,
}
