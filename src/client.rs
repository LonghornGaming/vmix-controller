use anyhow::{Context, Result};
use reqwest::blocking::RequestBuilder;
use log::{info, warn};
use crate::config::Config;
use crate::xml::Vmix;

use std::fs::File;
use std::io::prelude::*;

pub struct Client {
    api: String,
    client: reqwest::blocking::Client,
    state: Vmix,
}

impl Client {
    pub fn new(cfg: Config, dump_xml: bool) -> Result<Self> {
        info!("Initializing endpoint {}", cfg.endpoint);
        let client = reqwest::blocking::Client::new();

        let api =  format!("http://{}/api", cfg.endpoint);

        let response = client.get(&api).send()
                                        .with_context(|| "could not connect to vMix (check the IP-address and the port in vMix settings)".to_string())?
                                        .text()?;

        if dump_xml {
            let mut state_file = File::create("last_state.xml")?;
            state_file.write_all(&response.as_bytes())?;
        }

        let state: Vmix = quick_xml::de::from_str(&response)?;
        info!("{:#?}", &state);

        let major_version = state.version.split_once('.').unwrap().0.parse::<u32>()?;
        if major_version < 27 {
            warn!("vMix versions less that 27 are not supported");
        }

        Ok(Self {
            api,
            client,
            state
        })
    }
    fn call(&self, name: &str) -> RequestBuilder {
        self.client
            .get(&self.api)
            .query(&[("Function", name)])
    }

    pub fn start_streaming(&self) -> Result<()> {
        self.call("StartStreaming").send()?;
        Ok(())
    }

    pub fn quick_play(&self, input: &str) -> Result<()> {
        self.call("QuickPlay").query(&[
            ("Input", input)
        ]).send()?;
        Ok(())
    }

    pub fn cut_direct(&self, input: &str) -> Result<()> {
        self.call("CutDirect").query(&[
            ("Input", input)
        ]).send()?;
        Ok(())
    }
}