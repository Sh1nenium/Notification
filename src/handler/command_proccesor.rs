use std::str::FromStr;

use frankenstein::{Chat, Update, UpdateContent};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::BotError;

use super::client::ApiClient;

const BOT_NAME: &str = "@NotShinenBot";

pub enum Command {
    Start,
    UnknownCommand(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessUpdateTask {
    update: Update,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_str = s.replace(BOT_NAME, "");

        let result = match command_str.trim() {
            "/start" => Command::Start,
            _ => Command::UnknownCommand(command_str.to_string()),
        };

        Ok(result)
    }
}

#[derive(TypedBuilder)]
pub struct UpdateProcessor {
    pub api: &'static ApiClient,
    pub text: String,
    pub message_id: i32,
    pub username: String,
    pub command: Command,
    pub chat: Chat,
}

impl UpdateProcessor {
    pub async fn create(update: Update) -> Result<Self, BotError> {
        if let UpdateContent::Message(message) = &update.content {
            if message.text.is_none() {
                log::error!("Update doesn't contain any text {:?}", message);

                return Err(BotError::UpdateNotMessage("no text".to_string()));
            }

            let text = message.text.clone().unwrap();

            let api = ApiClient::api_client().await;

            let user = message.from.clone().expect("User not set");
            let chat = message.chat.clone();
            let username = match user.username {
                Some(name) => format!("@{}", name),
                None => user.first_name,
            };

            let command = Command::from_str(&text).unwrap();

            let processor = Self::builder()
                .api(api)
                .message_id(message.message_id)
                .text(text)
                .username(username)
                .chat(*chat)
                .command(command)
                .build();

            Ok(processor)
        } else {
            log::error!("Update is not a message {:?}", update);

            Err(BotError::UpdateNotMessage("no message".to_string()))
        }
    }
}
