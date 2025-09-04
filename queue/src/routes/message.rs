use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{message::Message, state::SharedState};

#[derive(Debug, Deserialize)]
pub struct MessageAddPayload {
    pub message: String,
    pub queue_name: String,
}
pub async fn add_message(State(state): State<SharedState>, Json(payload): Json<MessageAddPayload>) {
    let mut lock = state.write().await;
    lock.register
        .get_queue(payload.queue_name)
        .add_message(Message::new(payload.message));
}

#[derive(Debug, Deserialize)]
pub struct MessageFetchPayload {
    pub count: usize,
    pub queue_name: String,
}
pub async fn get_messages(
    State(state): State<SharedState>,
    Json(payload): Json<MessageFetchPayload>,
) -> Response {
    let mut lock = state.write().await;
    let messages = lock
        .register
        .get_queue(payload.queue_name)
        .get_messages(payload.count);

    (StatusCode::OK, Json(messages)).into_response()
}

#[derive(Debug, Deserialize)]
pub struct MessageIdPayload {
    queue_name: String,
    ids: Vec<String>,
}
pub async fn delete_messages(
    State(state): State<SharedState>,
    Json(payload): Json<MessageIdPayload>,
) {
    let mut lock = state.write().await;
    lock.register
        .get_queue(payload.queue_name)
        .delete_messages(payload.ids);
}

pub async fn retry_messages(
    State(state): State<SharedState>,
    Json(payload): Json<MessageIdPayload>,
) {
    let mut lock = state.write().await;
    let to_dlq = lock
        .register
        .get_queue(payload.queue_name)
        .retry_messages(payload.ids);

    for (dlq, message) in to_dlq {
        lock.register.get_queue(dlq).add_message(message);
    }
}
