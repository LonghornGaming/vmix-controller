#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vmix {
    pub version: String,
    pub inputs: InputList,
    // pub overlays: OverlayList,
    pub preview: u32,
    pub active: u32,
    #[serde(rename = "fadeToBlack")]
    pub ftb: bool,
    // pub transitions: TransitionList,
    pub recording: bool,
    pub external: bool,
    pub streaming: bool,
    #[serde(rename = "playList")]
    pub playlist: bool,
    #[serde(rename = "multiCorder")]
    pub multicorder: bool,
}

#[derive(Deserialize, Debug)]
pub struct InputList {
    #[serde(default)]
    pub input: Vec<Input>,
}
#[derive(Deserialize, Debug)]
pub struct OverlayList {
    #[serde(default)]
    pub overlay: Vec<Overlay>,
}
#[derive(Deserialize, Debug)]
pub struct TransitionList {
    #[serde(default)]
    pub transition: Vec<Transition>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Input {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@type")]
    pub kind: String,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@state")]
    pub state: String,
    pub text: Option<Vec<Text>>,
}

#[derive(Deserialize, Debug)]
pub struct Overlay {}

#[derive(Deserialize, Debug)]
pub struct Transition {}

#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    #[serde(rename = "@index")]
    pub index: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub val: String,
}
