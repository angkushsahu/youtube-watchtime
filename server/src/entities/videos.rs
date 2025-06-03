use serde::{Deserialize, Serialize};

use super::common::PageInfo;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSchema {
    pub kind: String,
    pub etag: String,
    pub items: Vec<VideoItem>,
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub content_details: VideoContentDetails,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoContentDetails {
    pub duration: String,
    pub dimension: String,
    pub definition: String,
    pub caption: String,
    pub licensed_content: bool,
    pub projection: String,
}
