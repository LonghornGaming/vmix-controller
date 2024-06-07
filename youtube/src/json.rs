#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LiveChatMessageListResponse {
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String, // provide to get the next page of events
    #[serde(rename = "pollingIntervalMillis")]
    pub polling_interval_millis: u32, // time to wait before polling api again
    pub items: Vec<LiveChatMessage>, // page of chat events
    // pub items: LiveChatMessageList, // page of chat events
}

#[derive(Deserialize, Debug)]
pub struct LiveChatMessageList {
    #[serde(default)]
    pub item: Vec<LiveChatMessage>,
}

#[derive(Deserialize, Debug)]
pub struct LiveChatMessage {
    pub id: String,       // event id
    pub snippet: ChatSnippet, // event
}

#[derive(Deserialize, Debug)]
pub struct ChatSnippet {
    #[serde(rename = "type")]
    pub kind: String, // event type
    #[serde(rename = "liveChatId")]
    pub id: String,
    #[serde(rename = "memberMilestoneChatDetails")]
    pub sub_milestone: Option<MemberMilestoneChatDetails>,
    #[serde(rename = "newSponsorDetails")]
    pub sub: Option<NewSponsorDetails>,
    #[serde(rename = "membershipGiftingDetails")]
    pub sub_gift: Option<MembershipGiftingDetails>,
    #[serde(rename = "giftMembershipReceivedDetails")]
    pub sub_gift_recipient: Option<GiftMembershipRecievedDetails>,
    #[serde(rename = "superChatDetails")]
    pub super_chat: Option<SuperChatDetails>,
    #[serde(rename = "superStickerDetails")]
    pub super_sticker: Option<SuperStickerDetails>,
}

// sub anniversary
#[derive(Deserialize, Debug)]
pub struct MemberMilestoneChatDetails {}

// sub
#[derive(Deserialize, Debug)]
pub struct NewSponsorDetails {}

// sub gift
#[derive(Deserialize, Debug)]
pub struct MembershipGiftingDetails {}

// gift sub recipient
#[derive(Deserialize, Debug)]
pub struct GiftMembershipRecievedDetails {}

// superchat
#[derive(Deserialize, Debug)]
pub struct SuperChatDetails {}

// supersticker
#[derive(Deserialize, Debug)]
pub struct SuperStickerDetails {}

#[derive(Deserialize, Debug)]
pub struct LiveBroadcastListResponse {
    items: Vec<LiveBroadcast>,
}

#[derive(Deserialize, Debug)]
pub struct LiveBroadcast {
    // pub id: String,
    pub snippet: BroadcastSnippet,
}

#[derive(Deserialize, Debug)]
pub struct BroadcastSnippet {
    #[serde(rename = "liveChatId")]
    pub live_chat_id: String,
}