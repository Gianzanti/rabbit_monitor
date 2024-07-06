use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RabbitResponse {
    pub items: Vec<Queue>,
}

#[derive(Debug, Deserialize)]
pub struct Queue {
    pub memory: u32,
    pub message_bytes: u32,
    pub messages: u32,
    pub messages_details: Details,
    pub messages_ready: u32,
    pub messages_ready_details: Details,
    pub messages_unacknowledged: u32,
    pub messages_unacknowledged_details: Details,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Details {
    pub rate: f64,
}
