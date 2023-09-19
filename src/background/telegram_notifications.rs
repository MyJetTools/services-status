use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::app_ctx::AppContext;

pub struct TelegramNotification {
    app: Arc<AppContext>,
}

impl TelegramNotification {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
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

        let snapshot = self.app.services_list.get_snapshot().await;

        let now = DateTimeAsMicroseconds::now();

        for (_, services) in snapshot {
            for service in services {
                let service_ok_duration = now.duration_since(service.last_ok_ping);

                if service_ok_duration.as_positive_or_zero().as_secs() > 60 {
                    let env_info = self.app.settings_reader.get_env_info().await;
                    crate::telegram_api::send_message(
                        &telegram_settings,
                        env_info.as_str(),
                        format!(
                            " Service {}:{} is not ok for {} seconds",
                            service.app_name,
                            service.app_version,
                            service_ok_duration.as_positive_or_zero().as_secs()
                        )
                        .as_str(),
                    )
                    .await;
                }
            }
        }
    }
}
