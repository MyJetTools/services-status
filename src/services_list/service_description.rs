use std::time::Duration;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::settings::ServiceSettings;

#[derive(Debug, Clone)]
pub struct ServiceDescription {
    pub domain: String,
    pub id: String,
    pub url: String,
    pub last_ok_ping: DateTimeAsMicroseconds,
    pub last_error: Option<String>,
    pub app_name: String,
    pub app_version: String,
    pub env_info: Option<String>,
    pub last_ping_duration: Duration,
    pub started: Option<i64>,
}

impl ServiceDescription {
    pub fn new(domain: String, settings: &ServiceSettings) -> Self {
        Self {
            id: settings.id.clone(),
            url: settings.url.clone(),
            last_ok_ping: DateTimeAsMicroseconds::now(),
            last_error: None,
            app_name: "???".to_string(),
            app_version: "???".to_string(),
            env_info: None,
            last_ping_duration: Duration::from_secs(0),
            started: None,
            domain,
        }
    }

    pub fn is_changed(&self, settings: &ServiceSettings) -> bool {
        self.url != settings.url
    }
}
