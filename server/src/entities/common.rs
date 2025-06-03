use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub default: ThumbNailsItem,
    pub medium: ThumbNailsItem,
    pub high: ThumbNailsItem,
    pub standard: ThumbNailsItem,
    pub maxres: ThumbNailsItem,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbNailsItem {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: u16,
    pub results_per_page: u8,
}
