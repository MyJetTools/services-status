use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "Services")]
    pub services: BTreeMap<String, Vec<ServiceSettings>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSettings {
    pub id: String,
    pub url: String,
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
        let settings = SettingsModel { services };

        let result = serde_yaml::to_string(&settings).unwrap();

        println!("{}", result);
    }
}
