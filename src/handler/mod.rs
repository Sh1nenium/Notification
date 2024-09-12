pub mod client;
pub mod command_proccesor;

use std::sync::Arc;

use command_proccesor::{Command, UpdateProcessor};
use frankenstein::{AsyncTelegramApi, ChatId, SendMessageParams};
use tokio::sync::Mutex;

use crate::AppState;

pub async fn poll_updates(state: Arc<Mutex<AppState>>) {
    loop {
        let mut state_lock = state.lock().await;
        let mut state = state_lock.telegram_api.clone();

        while let Some(update) = state.next_update().await {
            let event = UpdateProcessor::create(update).await;
            if let Ok(event) = event {
                if let Command::Start = event.command {
                    let message = SendMessageParams::builder()
                        .chat_id(ChatId::from(event.chat.id))
                        .text("Hi!")
                        .build();

                    state.telegram_client.send_message(&message).await.unwrap();

                    state_lock.users.insert(event.chat.id);
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
