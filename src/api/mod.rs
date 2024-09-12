use std::sync::Arc;

use axum::Router;
use telegram_route::TelegramRoute;
use tokio::sync::Mutex;

use crate::AppState;

pub mod telegram_route;

pub struct ApiRouter;

impl ApiRouter {
  pub fn new(state: Arc<Mutex<AppState>>) -> Router {
    let router = Router::new()
      .nest("/telegram", TelegramRoute::new(state));

    Router::new()
      .nest("/api", router)
  }
}