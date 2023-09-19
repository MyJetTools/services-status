use std::{sync::Arc, time::Duration};

use background::ServicesPinger;
use rust_extensions::MyTimer;

mod app_ctx;
mod background;
mod http;
mod services_list;
mod settings;
mod telegram_api;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".services-status").await;

    let settings_reader = Arc::new(settings_reader);

    let app = app_ctx::AppContext::new(settings_reader);

    let app = Arc::new(app);

    let mut http_server = http::start_up::setup_server(&app, 8000);

    if let Some(telegram_settings) = app.settings_reader.get_telegram_settings().await {
        telegram_api::send_message(&telegram_settings, "Service status app started".into()).await;
    } else {
        println!("telegram_api_key is not setup. Messages are not going to be sent");
    }

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());

    let mut timer_3s = MyTimer::new(Duration::from_secs(3));

    timer_3s.register_timer(
        "Services Pinger",
        Arc::new(ServicesPinger::new(app.clone())),
    );

    timer_3s.start(app.app_states.clone(), my_logger::LOGGER.clone());

    app.app_states.wait_until_shutdown().await;
}
