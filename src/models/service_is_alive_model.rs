use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceApiIsAliveModel {
    pub name: String,
    pub version: String,
    pub env_info: Option<String>,
    pub compiled: Option<String>,
    pub started: Option<i64>,
}
