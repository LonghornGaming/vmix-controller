pub mod client;
pub mod xml;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Input {
    Number(u32),
    Key(String),
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        let num = value.parse::<u32>();
        match num {
            Ok(num) => Self::Number(num),
            Err(_) => Self::Key(value),
        }
    }
}
