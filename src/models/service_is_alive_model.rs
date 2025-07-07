use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceApiIsAliveModel {
    pub name: String,
    pub version: String,
    pub compiled: Option<String>,
    pub started: Option<i64>,
}

//{"name":"admin-files","version":"0.1.0","env_info":"VM-01","started":1751715960471305,"compiled":"2025-07-05T11:44:18.540944+00:00"}
