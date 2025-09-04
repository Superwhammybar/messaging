use serde::Deserialize;

use crate::message::{Message, ProcessingMessage};

#[derive(Debug, Clone)]
pub struct Queue {
    pub name: String,
    pub id: String,
    pub dlq: Option<DLQConfig>,
    pub contents: Vec<Message>,
    pub processing: Vec<ProcessingMessage>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct DLQConfig {
    pub name: String,
    pub delivery_attempts: u8,
}

impl Queue {
    pub fn new(name: String, dlq_config: Option<DLQConfig>) -> Self {
        Queue {
            name,
            dlq: dlq_config,
            id: uuidv7::create(),
            contents: vec![],
            processing: vec![],
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.contents.push(message);
    }

    pub fn get_messages(&mut self, count: usize) -> Vec<Message> {
        let length = self.contents.len();
        let count = if count > length { length } else { count };
        let messages: Vec<Message> = self
            .contents
            .drain(0..count)
            .map(|mut m| {
                m.increment_attempts();
                m
            })
            .collect();
        for message in messages.clone() {
            self.processing.push(ProcessingMessage::from(message));
        }
        messages
    }

    pub fn delete_messages(&mut self, ids: Vec<String>) {
        self.processing.retain(|p| !ids.contains(&p.id));
    }

    pub fn retry_messages(&mut self, ids: Vec<String>) -> Vec<(String, Message)> {
        let mut to_dlq: Vec<(String, Message)> = vec![];
        let mut to_retry: Vec<Message> = vec![];
        for message in self.processing.clone() {
            if ids.contains(&message.id) {
                match &self.dlq {
                    Some(dlq) => {
                        if dlq.delivery_attempts <= message.delivery_attempts {
                            to_dlq.push((dlq.name.clone(), Message::from(message)));
                        }
                    }
                    None => {
                        to_retry.push(Message::from(message));
                    }
                }
            }
        }
        self.contents = [to_retry, self.contents.clone()].concat();
        self.processing.retain(|p| !ids.contains(&p.id));
        to_dlq
    }

    pub fn purge_queue(&mut self) {
        self.contents = vec![];
        self.processing = vec![];
    }
}
