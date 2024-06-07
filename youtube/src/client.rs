use crate::json;
use anyhow::{Context, Result};
use log::{info, warn};
use simd_json;
use crate::json::{LiveBroadcastListResponse, LiveChatMessageListResponse};

pub struct Client {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::blocking::Client::new();

        Ok(Self { api_key, client })
    }

    pub fn get_chat(&self, chat_id: &str, page_token: Option<String>) -> Result<LiveChatMessageListResponse> {
        let api = format!(
            "https://www.googleapis.com/youtube/v3/liveChat/messages?liveChatId={}&part=id,snippet",
            chat_id
        );
        // info!("[Youtube] Endpoint {}", &api);

        let builder = match page_token {
            None => self.client.get(api),
            Some(token) => self.client.get(api).query(&[("pageToken", token)]),
        };

        let mut response = builder
            .query(&["key", &self.api_key])
            .send()?
            .bytes()?
            .to_vec();

        Ok(simd_json::serde::from_slice::<LiveChatMessageListResponse>(&mut response).unwrap())
    }

    pub fn get_broadcasts(&self, page_token: Option<String>) -> Result<String> {
        // let api = "https://www.googleapis.com/youtube/v3/liveBroadcasts?part=id,snippet&mine=true";
        let api = "https://www.googleapis.com/youtube/v3/liveBroadcasts";

        // let builder = match page_token {
        //     None => self.client.get(api).query(&[("part", "id,snippet"), ("mine", "true")]),
        //     Some(token) => self.client.get(api).query(&[("part", "id,snippet"), ("mine", "true"), ("pageToken", &*token)]),
        // };

        let response = self.client
            .get(api)
            .query(&[("part", "snippet"), ("mine", "true")])
            .query(&[("key", &self.api_key)])
            .send()?;

        Ok(response.text()?)

        // let mut raw = response
        //     .bytes()?
        //     .to_vec();
        //
        // Ok(simd_json::serde::from_slice::<LiveBroadcastListResponse>(&mut raw).unwrap())
    }
}
