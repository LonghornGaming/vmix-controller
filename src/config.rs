use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::Commands;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: u8,
    pub endpoint: String,
    pub inputs: Inputs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inputs {
    vals: Vec<(Commands, Input)>,
}

impl Inputs {
    pub fn at(&self, cmd: Commands) -> Result<&Input> {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            endpoint: "127.0.0.1:8088".into(),
            inputs: Inputs {
                vals:  vec![
                    (Commands::Start, Input::Number(1)),
                    (Commands::Break, Input::Number(2)),
                    (Commands::Game, Input::Number(3)),
                    (Commands::End, Input::Number(4)),
                ]
            },
        }
    }
}