use axum::{Router, routing::post};
use broker::{
    routes::{
        message::{add_message, delete_messages, get_messages, retry_messages},
        queue::{add_queue, purge_queue, remove_queue},
    },
    state::SharedState,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let shared_state = SharedState::default();
    let app = Router::new()
        .route("/queue", post(add_queue).delete(remove_queue))
        .route("/queue/purge", post(purge_queue))
        .route("/message/add", post(add_message))
        .route("/message/fetch", post(get_messages))
        .route("/message/remove", post(delete_messages))
        .route("/message/retry", post(retry_messages))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
