use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, services_list::ServiceDescription};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServicesStatusResponse {
    pub env: String,
    pub vm: String,
    pub services: Vec<ServiceStatus>,
}

impl ServicesStatusResponse {
    pub async fn new(app: &AppContext) -> Self {
        let (env, vm) = app
            .settings_reader
            .use_settings(|settings| (settings.env.to_string(), settings.vm.to_string()))
            .await;
        let snapshot = app.services_list.get_snapshot().await;

        Self {
            services: snapshot
                .into_iter()
                .map(|service| ServiceStatus::from(service.1))
                .collect(),
            env,
            vm,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceStatus {
    pub id: String,
    pub name: String,
    pub version: String,
    pub compiled_at: Option<String>,
    pub last_ok: Option<usize>,
    pub last_error: Option<String>,
    pub last_ping_duration: String,
    pub started: Option<String>,
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

        let compiled_at = src.compiled.map(|mut dt| {
            while dt.len() > 19 {
                dt.pop();
            }
            dt
        });

        Self {
            id: src.id,
            name: src.app_name,
            version: src.app_version,

            compiled_at,
            last_ok,
            last_error: src.last_error,
            last_ping_duration: format!("{:?}", src.last_ping_duration),
            started: src.started.map(|itm| {
                let mut result = DateTimeAsMicroseconds::new(itm).to_rfc3339();

                while result.len() > 19 {
                    result.pop();
                }
                result
            }),
        }
    }
}
