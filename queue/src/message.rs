use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub body: String,
    pub timestamp: u64,
    pub delivery_attempts: u8,
}

impl Message {
    pub fn new(body: String) -> Message {
        Message {
            id: uuidv7::create(),
            body,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            delivery_attempts: 0,
        }
    }

    pub fn increment_attempts(&mut self) {
        self.delivery_attempts += 1;
    }
}

impl From<ProcessingMessage> for Message {
    fn from(value: ProcessingMessage) -> Self {
        Message {
            id: value.id,
            body: value.body,
            timestamp: value.timestamp,
            delivery_attempts: value.delivery_attempts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessingMessage {
    pub id: String,
    pub body: String,
    pub timestamp: u64,
    pub delivery_attempts: u8,
    pub visibility_timeout: u64,
}
impl From<Message> for ProcessingMessage {
    fn from(value: Message) -> Self {
        ProcessingMessage {
            id: value.id,
            body: value.body,
            timestamp: value.timestamp,
            delivery_attempts: value.delivery_attempts,
            visibility_timeout: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 30,
        }
    }
}
