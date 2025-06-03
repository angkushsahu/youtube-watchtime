use axum::http::StatusCode;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    entities::{
        playlist_items::PlaylistItemSchema,
        response::{ServerResponse, Videos},
    },
    utils::error::ApiError,
};

pub async fn playlist_items(
    api_key: &str,
    playlist_id: &str,
    server_response: &mut ServerResponse,
) -> Result<Vec<String>, ApiError> {
    let mut params = HashMap::new();
    params.insert("part", String::from("snippet"));
    params.insert("maxResults", String::from("50"));
    params.insert("key", String::from(api_key));
    params.insert("playlistId", String::from(playlist_id));

    let mut video_ids: Vec<String> = Vec::new();

    loop {
        let response = Client::new()
            .get("https://www.googleapis.com/youtube/v3/playlistItems")
            .query(&params)
            .send()
            .await
            .unwrap();
        let data: PlaylistItemSchema = response.json().await.map_err(|_| {
            ApiError::new(
                vec![String::from("Invalid response for 'Playlist Items' was returned from youtube")],
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

        server_response.playlist.total_videos = data.page_info.total_results;

        for video in data.items {
            server_response.videos.push(Videos {
                duration: String::new(),
                published_at: video.snippet.published_at,
                position: video.snippet.position + 1,
                id: video.snippet.resource_id.video_id.to_string(),
                thumbnails: video.snippet.thumbnails,
                title: video.snippet.title,
            });

            video_ids.push(video.snippet.resource_id.video_id);
        }

        if data.next_page_token.is_none() {
            break;
        }

        if let Some(next_page_token) = data.next_page_token {
            params.insert("pageToken", next_page_token);
        }
    }

    Ok(video_ids)
}
