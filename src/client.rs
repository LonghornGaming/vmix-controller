use anyhow::{Context, Result};
use reqwest::blocking::RequestBuilder;
use log::{info};
use crate::config::Config;
use crate::xml::Vmix;

pub struct Client {
    api: String,
    client: reqwest::blocking::Client,
    state: Vmix,
}

impl Client {
    pub fn new(cfg: Config) -> Result<Self> {
        info!("Initializing endpoint {}", cfg.endpoint);
        let client = reqwest::blocking::Client::new();

        let api =  format!("http://{}/api", cfg.endpoint);

        let state = quick_xml::de::from_str(
            &*client.get(&api).send()
            .with_context(|| "could not connect to vMix (check the IP-address and the port in vMix settings)".to_string())?
            .text()?
        )?;

        info!("{:#?}", &state);

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

    pub fn quick_play(&self, input: &Option<String>) -> Result<()> {
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