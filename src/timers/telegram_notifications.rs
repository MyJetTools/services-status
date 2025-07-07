use std::{collections::HashMap, sync::Arc};

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};
use tokio::sync::Mutex;

use crate::{app_ctx::AppContext, telegram_api::MessageType};

pub struct TelegramNotification {
    app: Arc<AppContext>,
    errs: Mutex<HashMap<String, ()>>,
}

impl TelegramNotification {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self {
            app,
            errs: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for TelegramNotification {
    async fn tick(&self) {
        let settings = self.app.settings_reader.get_telegram_settings().await;

        if settings.is_none() {
            return;
        }

        let telegram_settings = settings.unwrap();

        let mut errs = self.errs.lock().await;

        let snapshot = self.app.services_list.get_snapshot().await;

        let now = DateTimeAsMicroseconds::now();

        let env_info = self
            .app
            .settings_reader
            .use_settings(|itm| itm.env.clone())
            .await;

        for service in snapshot.values() {
            let service_ok_duration = if let Some(last_ok_ping) = service.last_ok_ping {
                now.duration_since(last_ok_ping).get_full_seconds()
            } else {
                0
            };

            if service_ok_duration > 60 {
                errs.insert(service.app_name.clone(), ());
                crate::telegram_api::send_message(
                    &telegram_settings,
                    env_info.as_str(),
                    MessageType::ServiceIsDown,
                    format!(
                        "[{}]. Service {}:{} is not ok for {} seconds",
                        service.id, service.app_name, service.app_version, service_ok_duration
                    )
                    .as_str(),
                )
                .await;
            } else {
                if let Some(_) = errs.remove(&service.app_name) {
                    crate::telegram_api::send_message(
                        &telegram_settings,
                        env_info.as_str(),
                        MessageType::ServiceIsUp,
                        format!(
                            "Service {}:{} is ok now",
                            service.app_name, service.app_version
                        )
                        .as_str(),
                    )
                    .await;
                }
            }
        }
    }
}
