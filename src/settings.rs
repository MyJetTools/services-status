use serde_derive::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub env: String,
    pub vm: String,
    pub unix_sockets_path: String,
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
