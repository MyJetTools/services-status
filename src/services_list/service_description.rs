use std::time::Duration;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::models::ServiceApiIsAliveModel;

#[derive(Debug, Clone)]
pub struct ServiceDescription {
    pub id: String,

    pub compiled: Option<String>,
    pub last_ok_ping: Option<DateTimeAsMicroseconds>,
    pub last_error: Option<String>,
    pub app_name: String,
    pub app_version: String,
    pub last_ping_duration: Duration,
    pub started: Option<i64>,
}

impl ServiceDescription {
    pub fn new_as_ok(
        id: String,
        settings: ServiceApiIsAliveModel,
        ping_duration: Duration,
    ) -> Self {
        Self {
            id: id,
            compiled: settings.compiled,
            last_ok_ping: Some(DateTimeAsMicroseconds::now()),
            last_error: None,
            app_name: settings.name,
            app_version: settings.version,
            last_ping_duration: ping_duration,
            started: settings.started,
        }
    }

    pub fn new_as_err(id: String, err: String, ping_duration: Duration) -> Self {
        Self {
            id: id,
            compiled: None,
            last_ok_ping: None,
            last_error: Some(err),
            app_name: "???".to_string(),
            app_version: "???".to_string(),
            last_ping_duration: ping_duration,
            started: None,
        }
    }

    pub fn update(&mut self, description: ServiceApiIsAliveModel, ping_duration: Duration) {
        self.last_ping_duration = ping_duration;
        self.app_name = description.name;
        self.app_version = description.version;
        self.last_error = None;
        self.compiled = description.compiled;
        self.started = description.started;
        self.last_ok_ping = Some(DateTimeAsMicroseconds::now());
    }
}
