use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: u8,
    pub endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            endpoint: "127.0.0.1:8088".into(),
        }
    }
}