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
        let snapshot = app.services_list.get_snapshot().await;

        let mut services = BTreeMap::new();

        for (domain, domain_services) in snapshot {
            let mut group = Vec::with_capacity(services.len());

            for service in domain_services {
                group.push(ServiceStatus::from(service));
            }

            services.insert(domain, group);
        }

        Self { services }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceStatus {
    pub id: String,
    pub name: String,
    pub url: String,
    pub version: String,
    #[serde(rename = "compiledAt")]
    pub compiled_at: String,
    #[serde(rename = "lastOk")]
    pub last_ok: usize,
    #[serde(rename = "lastError")]
    pub last_error: Option<String>,
    #[serde(rename = "lastPingDuration")]
    pub last_ping_duration: String,
    #[serde(rename = "envInfo")]
    pub env_info: Option<String>,
    pub started: Option<i64>,
}

impl ServiceStatus {
    pub fn from(src: ServiceDescription) -> Self {
        let now = DateTimeAsMicroseconds::now();

        Self {
            id: src.id,
            name: src.app_name,
            version: src.app_version,
            url: src.url,
            compiled_at: src.compiled.unwrap_or_else(|| "".to_string()),
            last_ok: now
                .duration_since(src.last_ok_ping)
                .as_positive_or_zero()
                .as_secs() as usize,
            last_error: src.last_error,
            last_ping_duration: format!("{:?}", src.last_ping_duration),
            env_info: src.env_info,
            started: src.started,
        }
    }
}
