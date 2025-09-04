use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::queue::Queue;

#[derive(Default, Debug)]
pub struct Register {
    queues: HashMap<String, Queue>,
}

impl Register {
    pub fn handle_timeouts(&mut self) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for mut queue in self.queues.clone().into_iter() {
            let mut to_retry: Vec<String> = vec![];
            queue.1.processing.iter().for_each(|m| {
                if timestamp > m.visibility_timeout {
                    to_retry.push(m.id.clone())
                }
            });

            queue.1.retry_messages(to_retry);
        }
    }
    pub fn register_queue(&mut self, queue: Queue) {
        self.queues.entry(queue.name.clone()).or_insert(queue);
    }

    pub fn deregister_queue(&mut self, name: String) {
        self.queues.remove(&name);
    }

    pub fn get_queue(&mut self, name: String) -> &mut Queue {
        self.queues.get_mut(&name).unwrap()
    }
}
