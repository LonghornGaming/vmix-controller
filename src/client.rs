#![allow(dead_code)]

use crate::config::Config;
use crate::config::Input;
use crate::xml;
use crate::xml::Vmix;
use anyhow::{Context, Result};
use log::{info, warn};
use reqwest::blocking::RequestBuilder;

pub struct Client {
    api: String,
    client: reqwest::blocking::Client,
    xml: String,
    state: Option<Vmix>
}

impl Client {
    pub fn new(cfg: &Config) -> Result<Self> {
        info!("Initializing endpoint {}", cfg.endpoint);
        let client = reqwest::blocking::Client::new();

        let api = format!("http://{}/api", cfg.endpoint);

        let xml = client
            .get(&api)
            .send()
            .with_context(|| {
                "could not connect to vMix (check the IP-address and the port in vMix settings)"
                    .to_string()
            })?
            .text()?;

        let state = None;

        // let major_version = state.version.split_once('.').unwrap().0.parse::<u32>()?;
        // if major_version < 27 {
        //     warn!("vMix versions less that 27 are not supported");
        // }

        Ok(Self { api, client, xml, state })
    }

    pub fn xml(&self) -> Result<&String> {
        Ok(&self.xml)
    }

    // Only parse XML when actually necessary
    fn state(&mut self) -> Result<&Vmix> {
        if let None = &self.state {
            self.state = Some(quick_xml::de::from_str(&self.xml)?);
        };

        match &self.state {
            Some(state) => Ok(&state),
            None => unreachable!(),
        }
    }

    pub fn inputs(&mut self) -> Result<&[xml::Input]> {
        Ok(self.state()?.inputs.input.as_slice())
    }

    pub fn titles(&mut self) -> Result<Vec<&xml::Input>> {
        Ok(self.inputs()?.iter().filter(|i| i.kind == "GT").collect())
    }

    fn call(&self, name: &str) -> RequestBuilder {
        self.client.get(&self.api).query(&[("Function", name)])
    }

    pub fn start_streaming(&self) -> Result<()> {
        self.call("StartStreaming").send()?;
        Ok(())
    }

    pub fn quick_play(&self, input: &Input) -> Result<()> {
        self.call("QuickPlay")
            .query(&[(
                "Input",
                match input {
                    Input::Key(key) => key.clone(),
                    Input::Number(num) => num.to_string(),
                },
            )])
            .send()?;
        Ok(())
    }

    pub fn cut_direct(&self, input: &Input) -> Result<()> {
        self.call("CutDirect")
            .query(&[(
                "Input",
                match input {
                    Input::Key(key) => key.clone(),
                    Input::Number(num) => num.to_string(),
                },
            )])
            .send()?;
        Ok(())
    }

    pub fn set_text(&self, input: &Input, idx: Option<u32>, value: String) -> Result<()> {
        let new = self.call("SetText").query(&[
            (
                "Input",
                match input {
                    Input::Key(key) => key.clone(),
                    Input::Number(num) => num.to_string(),
                },
            ),
            ("Value", value),
        ]);

        match idx {
            None => new.send()?,
            Some(i) => new.query(&[("SelectedIndex", i.to_string())]).send()?,
        };

        Ok(())
    }

    pub fn get(&mut self, input: &Input) -> Result<&xml::Input> {
        let val = self
            .inputs()?
            .iter()
            .filter(|t| match &input {
                &Input::Number(num) => t.number.parse::<u32>().unwrap() == *num,
                &Input::Key(key) => t.key == *key || t.title == *key,
            })
            .nth(0)
            .unwrap();

        Ok(val)
    }

    pub fn get_text(&mut self, input: &Input, idx: Option<u32>) -> Result<String> {
        let prev_text = &self.get(input)?.text.as_ref().unwrap()[match idx {
            Some(i) => i as usize,
            None => 0,
        }];

        Ok(prev_text.val.clone())
    }
}
