use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::SwitchCommands;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: u8,
    pub endpoint: String,
    pub inputs: Inputs,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            endpoint: "127.0.0.1:8088".into(),
            inputs: Inputs {
                vals: vec![
                    (SwitchCommands::Start, Input::Number(1)),
                    (SwitchCommands::Break, Input::Number(2)),
                    (SwitchCommands::Game, Input::Number(3)),
                    (SwitchCommands::End, Input::Number(4)),
                ],
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Inputs {
    vals: Vec<(SwitchCommands, Input)>,
}

impl Inputs {
    pub fn at(&self, cmd: SwitchCommands) -> Result<&Input> {
        let val = &self.vals[cmd as usize];

        assert_eq!(cmd, val.0);
        Ok(&val.1)
    }
}

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
