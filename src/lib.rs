use std::collections::HashSet;

use handler::client::ApiClient;
use thiserror::Error;

pub mod api;
pub mod handler;

pub use handler::poll_updates;

#[derive(Clone)]
pub struct AppState {
    pub telegram_api: &'static ApiClient,
    pub users: HashSet<i64>,
}


#[derive(Debug, Error)]
pub enum BotError {
    #[error(transparent)]
    MessageError(#[from] std::fmt::Error),
    #[error("Update can not be processed {}", self)]
    UpdateNotMessage(String),
}
