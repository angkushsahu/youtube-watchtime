use super::common::Thumbnails;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub published_at: String,
    pub title: String,
    pub thumbnails: Thumbnails,
    pub total_videos: u16,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Videos {
    pub duration: String,
    pub published_at: String,
    pub position: u16,
    pub id: String,
    pub thumbnails: Thumbnails,
    pub title: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerResponse {
    pub channel_name: String,
    pub playlist: Playlist,
    pub videos: Vec<Videos>,
}
