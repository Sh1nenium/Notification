use std::{collections::HashSet, sync::Arc};

use axum::{
    routing::get,
    Router,
};
use log::info;
use notification::{api::ApiRouter, handler::client::ApiClient, poll_updates, AppState};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let app_state = Arc::new(Mutex::new(AppState {
        telegram_api: ApiClient::api_client().await,
        users: HashSet::new(),
    }));

    let app_state_handler = app_state.clone();
    let api_state_route: Arc<Mutex<AppState>> = app_state.clone();

    tokio::spawn(async move {
        poll_updates(app_state_handler).await;
    });

    let api: Router = ApiRouter::new(api_state_route);

    let app: Router = Router::new()
        .route("/", get(|| async { "Notification service" }))
        .merge(api);

    let port = std::env::var("PORT").unwrap_or("8100".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect(format!("Failed to bind to 0.0.0.0:{}", port).as_str());
    info!("Listening on 0.0.0.0:{}", port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}


