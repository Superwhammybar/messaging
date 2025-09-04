use std::sync::Arc;

use tokio::sync::RwLock;

use crate::register::Register;

pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Debug, Default)]
pub struct AppState {
    pub register: Register,
}
