use serde::{Deserialize, Serialize};

use super::common::{PageInfo, Thumbnails};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResponse {
    pub kind: String,
    pub etag: String,
    pub page_info: PageInfo,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: PlaylistSnippet,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSnippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub channel_title: String,
    pub localized: PlaylistLocalized,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistLocalized {
    pub title: String,
    pub description: String,
}
