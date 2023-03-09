use std::{collections::BTreeMap, time::Duration};

use rust_extensions::{date_time::DateTimeAsMicroseconds, lazy::LazyVec};
use tokio::sync::Mutex;

use crate::settings::ServiceSettings;

use super::ServiceDescription;

pub struct ServicesList {
    pub services: Mutex<BTreeMap<String, BTreeMap<String, ServiceDescription>>>,
}

impl ServicesList {
    pub fn new() -> Self {
        Self {
            services: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn sync_services_list(&self, settings: &BTreeMap<String, Vec<ServiceSettings>>) {
        let mut write_access = self.services.lock().await;

        for (domain, settings) in settings {
            if let Some(service_description) = write_access.get_mut(domain) {
                apply_changes(domain, service_description, settings);
            } else {
                let mut service_set = Vec::with_capacity(settings.len());
                for setting in settings {
                    service_set.push((
                        setting.id.clone(),
                        ServiceDescription::new(domain.to_string(), setting),
                    ));
                }

                write_access.insert(domain.to_string(), service_set.into_iter().collect());
            }
        }

        let mut domains_to_delete = LazyVec::new();

        for domain in write_access.keys() {
            if !settings.contains_key(domain) {
                domains_to_delete.add(domain.to_string());
            }
        }

        if let Some(domains_to_delete) = domains_to_delete.get_result() {
            for domain in domains_to_delete {
                write_access.remove(&domain);
            }
        }
    }

    pub async fn update_ok(
        &self,
        domain: &str,
        id: &str,
        name: String,
        version: String,
        env_info: Option<String>,
        started: Option<i64>,
        last_ping_duration: Duration,
    ) {
        let mut write_access = self.services.lock().await;

        if let Some(group) = write_access.get_mut(domain) {
            if let Some(item) = group.get_mut(id) {
                item.last_ok_ping = DateTimeAsMicroseconds::now();
                item.app_name = name;
                item.app_version = version;
                item.last_error = None;
                item.last_ping_duration = last_ping_duration;
                item.env_info = env_info;
                item.started = started;
            }
        }
    }

    pub async fn update_error(
        &self,
        domain: &str,
        id: &str,
        error: String,
        last_ping_duration: Duration,
    ) {
        let mut write_access = self.services.lock().await;

        if let Some(group) = write_access.get_mut(domain) {
            if let Some(item) = group.get_mut(id) {
                item.last_error = Some(error);
                item.last_ping_duration = last_ping_duration;
            }
        }
    }

    pub async fn get_snapshot(&self) -> BTreeMap<String, Vec<ServiceDescription>> {
        let mut result = BTreeMap::new();
        let read_access = self.services.lock().await;

        for (domain, services) in &*read_access {
            let services = services.values().cloned().collect::<Vec<_>>();
            result.insert(domain.to_string(), services);
        }

        result
    }
}

fn apply_changes<'s>(
    domain: &str,
    services: &mut BTreeMap<String, ServiceDescription>,
    service_settings: &'s [ServiceSettings],
) {
    let mut has_to_be_updated = LazyVec::new();

    for service_setting in service_settings {
        if let Some(service_description) = services.get(&service_setting.id) {
            if service_description.is_changed(service_setting) {
                has_to_be_updated.add(ServiceDescription::new(domain.to_string(), service_setting));
            }
        } else {
            services.insert(
                service_setting.id.clone(),
                ServiceDescription::new(domain.to_string(), service_setting),
            );
        }
    }

    if let Some(has_to_be_updated) = has_to_be_updated.get_result() {
        for service_description in has_to_be_updated {
            services.insert(service_description.id.clone(), service_description);
        }
    }

    let mut to_delete = LazyVec::new();

    for service in services.values() {
        if !service_settings.iter().any(|s| s.id == service.id) {
            to_delete.add(service.id.clone());
        }
    }

    if let Some(to_delete) = to_delete.get_result() {
        for id in to_delete {
            services.remove(&id);
        }
    }
}
