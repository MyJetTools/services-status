use rust_extensions::AppStates;

use crate::services_list::ServicesList;

use std::sync::Arc;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub settings_reader: Arc<crate::settings::SettingsReader>,
    pub app_states: Arc<AppStates>,
    pub services_list: ServicesList,
    pub process_id: String,
}

impl AppContext {
    pub fn new(settings_reader: Arc<crate::settings::SettingsReader>) -> AppContext {
        AppContext {
            app_states: Arc::new(AppStates::create_initialized()),
            services_list: ServicesList::new(),
            settings_reader,
            process_id: rust_extensions::SortableId::generate().to_string(),
        }
    }
}
