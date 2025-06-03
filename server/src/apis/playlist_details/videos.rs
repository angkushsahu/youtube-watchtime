use axum::http::StatusCode;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    entities::{response::ServerResponse, videos::VideoSchema},
    utils::error::ApiError,
};

pub async fn videos(
    api_key: &str,
    video_ids: Vec<String>,
    server_response: &mut ServerResponse,
) -> Result<(), ApiError> {
    let mut params = HashMap::new();
    params.insert("part", String::from("contentDetails"));
    params.insert("key", String::from(api_key));

    for chunk in video_ids.chunks(50) {
        let first_50_ids = chunk.join(",");
        params.insert("id", first_50_ids);

        let response = Client::new()
            .get("https://www.googleapis.com/youtube/v3/videos")
            .query(&params)
            .send()
            .await
            .unwrap();
        let data: VideoSchema = response.json().await.map_err(|_| {
            ApiError::new(
                vec![String::from("Invalid response for 'Videos' was returned from youtube")],
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

        let video_details: HashMap<String, String> = data
            .items
            .into_iter()
            .map(|item| (item.id.clone(), item.content_details.duration.clone()))
            .collect();

        for video in &mut server_response.videos {
            if let Some(duration) = video_details.get(&video.id) {
                video.duration = duration.to_string();
            }
        }
    }

    Ok(())
}
