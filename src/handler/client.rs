use frankenstein::AllowedUpdate;
use frankenstein::AsyncApi;
use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::Update;
use std::collections::VecDeque;
use tokio::sync::OnceCell;

static API_CLIENT: OnceCell<ApiClient> = OnceCell::const_new();

#[derive(Clone)]
pub struct ApiClient {
    pub telegram_client: AsyncApi,
    pub update_params: GetUpdatesParams,
    pub buffer: VecDeque<Update>,
}

impl ApiClient {
    pub async fn api_client() -> &'static Self {
        API_CLIENT.get_or_init(ApiClient::new).await
    }

    pub async fn new() -> Self {
        let token = std::env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set");
        let telegram_client = AsyncApi::new(&token);

        let update_params = GetUpdatesParams::builder()
            .allowed_updates(vec![AllowedUpdate::Message, AllowedUpdate::ChannelPost])
            .build();

        let buffer = VecDeque::new();

        Self {
            telegram_client,
            update_params,
            buffer,
        }
    }

    pub async fn next_update(&mut self) -> Option<Update> {
        if let Some(update) = self.buffer.pop_front() {
            return Some(update);
        }

        match self.telegram_client.get_updates(&self.update_params).await {
            Ok(updates) => {
                for update in updates.result {
                    self.buffer.push_back(update);
                }

                if let Some(last_update) = self.buffer.back() {
                    self.update_params.offset = Some((last_update.update_id + 1).into());
                }

                self.buffer.pop_front()
            }

            Err(err) => {
                log::error!("Failed to fetch updates {:?}", err);
                None
            }
        }
    }
}
