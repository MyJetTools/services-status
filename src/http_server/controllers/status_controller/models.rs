use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, services_list::ServiceDescription};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServicesStatusResponse {
    pub services: Vec<ServiceStatus>,
}

impl ServicesStatusResponse {
    pub async fn new(app: &AppContext) -> Self {
        let snapshot = app.services_list.get_snapshot().await;

        Self {
            services: snapshot
                .into_iter()
                .map(|service| ServiceStatus::from(service.1))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceStatus {
    pub id: String,
    pub name: String,
    pub version: String,
    pub compiled_at: String,
    pub last_ok: Option<usize>,
    pub last_error: Option<String>,
    pub last_ping_duration: String,
    pub started: Option<i64>,
}

impl ServiceStatus {
    pub fn from(src: ServiceDescription) -> Self {
        let now = DateTimeAsMicroseconds::now();

        let last_ok = if let Some(last_ok_ping) = src.last_ok_ping {
            let result = now
                .duration_since(last_ok_ping)
                .as_positive_or_zero()
                .as_secs() as usize;

            Some(result)
        } else {
            None
        };

        Self {
            id: src.id,
            name: src.app_name,
            version: src.app_version,

            compiled_at: src.compiled.unwrap_or_else(|| "".to_string()),
            last_ok,
            last_error: src.last_error,
            last_ping_duration: format!("{:?}", src.last_ping_duration),
            started: src.started,
        }
    }
}
