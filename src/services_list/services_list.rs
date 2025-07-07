use std::{collections::BTreeMap, time::Duration};

use tokio::sync::Mutex;

use crate::models::ServiceApiIsAliveModel;

use super::ServiceDescription;

pub struct ServicesList {
    pub services: Mutex<BTreeMap<String, ServiceDescription>>,
}

impl ServicesList {
    pub fn new() -> Self {
        Self {
            services: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn update_service_ok(
        &self,
        id: String,
        data: ServiceApiIsAliveModel,
        ping_duration: Duration,
    ) {
        let mut write_access = self.services.lock().await;
        match write_access.get_mut(&id) {
            Some(value) => {
                value.update(data, ping_duration);
            }
            None => {
                write_access.insert(
                    id.to_string(),
                    ServiceDescription::new_as_ok(id, data, ping_duration),
                );
            }
        }
    }
    pub async fn update_error(&self, id: String, error: String, ping_duration: Duration) {
        let mut write_access = self.services.lock().await;

        match write_access.get_mut(&id) {
            Some(item) => {
                item.last_error = Some(error);
                item.last_ping_duration = ping_duration;
            }
            None => {
                write_access.insert(
                    id.to_string(),
                    ServiceDescription::new_as_err(id, error, ping_duration),
                );
            }
        }
    }

    pub async fn get_snapshot(&self) -> BTreeMap<String, ServiceDescription> {
        let read_access = self.services.lock().await;
        read_access.clone()
    }
}
