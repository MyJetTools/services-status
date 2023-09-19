use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "Services")]
    pub services: BTreeMap<String, Vec<ServiceSettings>>,
    #[serde(rename = "Telegram")]
    pub telegram: Option<TelegramSettings>,
}

impl SettingsReader {
    pub async fn get_telegram_settings(&self) -> Option<TelegramSettings> {
        let read_access = self.settings.read().await;
        read_access.telegram.clone()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSettings {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramSettings {
    pub api_key: String,
    pub chat_id: i64,
    pub message_thread_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut services = BTreeMap::new();
        services.insert(
            "domain".to_string(),
            vec![
                ServiceSettings {
                    id: "id".to_string(),
                    url: "url".to_string(),
                },
                ServiceSettings {
                    id: "id1".to_string(),
                    url: "url1".to_string(),
                },
            ],
        );
        let settings = SettingsModel {
            services,
            telegram: TelegramSettings {
                api_key: "xxx".to_string(),
                chat_id: 12,
                message_thread_id: 13,
            }
            .into(),
        };

        let result = serde_yaml::to_string(&settings).unwrap();

        println!("{}", result);
    }
}
