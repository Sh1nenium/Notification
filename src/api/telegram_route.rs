use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use frankenstein::{AsyncTelegramApi, ChatId, SendMessageParams};
use log::info;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

use crate::AppState;

pub struct TelegramRoute;

impl TelegramRoute {  
  pub fn new(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
      .route("/", post(Self::send_to_telegram))
      .with_state(state)
  }

  pub async fn send_to_telegram(
    State(telegram_api): State<Arc<Mutex<AppState>>>,
    Json(body): Json<Value>,
  ) {
    info!("Received request: {:?}", body);
    let tg_api = telegram_api.lock().await;
    let vec: Vec<&i64> = tg_api.users.iter().collect::<Vec<&i64>>();

    let mut stream = tokio_stream::iter(vec);

    let mut message = SendMessageParams::builder()
        .chat_id(ChatId::from(0))
        .text("Hello, this is notification service")
        .build();

    while let Some(user) = stream.next().await {
        message.chat_id = ChatId::from(*user);
        tg_api
            .telegram_api
            .telegram_client
            .send_message(&message)
            .await
            .unwrap();
    }
  }
}