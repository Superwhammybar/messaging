use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;

use crate::{
    queue::{DLQConfig, Queue},
    state::SharedState,
};

#[derive(Debug, Deserialize)]
pub struct QueueCreatePayload {
    name: String,
    dlq: Option<DLQConfig>,
}
#[derive(Debug, Deserialize)]
pub struct QueuePayload {
    name: String,
}

pub async fn add_queue(State(state): State<SharedState>, Json(queue): Json<QueueCreatePayload>) {
    let mut lock = state.write().await;
    lock.register
        .register_queue(Queue::new(queue.name, queue.dlq));
}
pub async fn remove_queue(State(state): State<SharedState>, Path(name): Path<String>) {
    let mut lock = state.write().await;
    lock.register.deregister_queue(name);
}
pub async fn purge_queue(State(state): State<SharedState>, Json(queue): Json<QueuePayload>) {
    let mut lock = state.write().await;
    lock.register.get_queue(queue.name).purge_queue();
}
