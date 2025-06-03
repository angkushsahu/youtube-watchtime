use serde::{Deserialize, Serialize};

use super::common::{PageInfo, Thumbnails};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSchema {
    pub kind: String,
    pub etag: String,
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub items: Vec<PlaylistItemItem>,
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemItem {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: PlaylistItemSnippet,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub channel_title: String,
    pub playlist_id: String,
    pub position: u16,
    pub resource_id: PlaylistItemResourceID,
    pub video_owner_channel_title: String,
    pub video_owner_channel_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemResourceID {
    pub kind: String,
    pub video_id: String,
}
