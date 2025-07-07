use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, services_list::ServiceDescription};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServicesStatusResponse {
    pub services: BTreeMap<String, Vec<ServiceStatus>>,
}

impl ServicesStatusResponse {
    pub async fn new(app: &AppContext) -> Self {
        let env_info = app
            .settings_reader
            .use_settings(|itm| itm.env.to_string())
            .await;
        let snapshot = app.services_list.get_snapshot().await;

        let mut services = BTreeMap::new();

        for service in snapshot {
            let mut group = Vec::with_capacity(services.len());

            group.push(ServiceStatus::from(service.1, &env_info));

            services.insert(service.0, group);
        }

        Self { services }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceStatus {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "compiledAt")]
    pub compiled_at: String,
    #[serde(rename = "lastOk")]
    pub last_ok: Option<usize>,
    #[serde(rename = "lastError")]
    pub last_error: Option<String>,
    #[serde(rename = "lastPingDuration")]
    pub last_ping_duration: String,
    #[serde(rename = "envInfo")]
    pub env_info: String,
    pub started: Option<i64>,
}

impl ServiceStatus {
    pub fn from(src: ServiceDescription, env_info: &str) -> Self {
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
            env_info: env_info.to_string(),
            started: src.started,
        }
    }
}
