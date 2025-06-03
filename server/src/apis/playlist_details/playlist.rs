use axum::http::StatusCode;
use reqwest::Client;
use std::collections::HashMap;

use crate::{
    entities::{playlist::PlaylistResponse, response::ServerResponse},
    utils::error::ApiError,
};

pub async fn playlist(
    api_key: &str,
    playlist_id: &str,
    server_response: &mut ServerResponse,
) -> Result<(), ApiError> {
    let mut params = HashMap::new();
    params.insert("part", String::from("snippet"));
    params.insert("key", String::from(api_key));
    params.insert("id", String::from(playlist_id));

    let response = Client::new()
        .get("https://www.googleapis.com/youtube/v3/playlists")
        .query(&params)
        .send()
        .await
        .unwrap();
    let data: PlaylistResponse = response.json().await.map_err(|_| {
        ApiError::new(
            vec![String::from("Invalid response for 'Playlist' was returned from youtube")],
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    server_response.channel_name = data.items[0].snippet.channel_title.to_string();
    server_response.playlist.published_at = data.items[0].snippet.published_at.to_string();
    server_response.playlist.title = data.items[0].snippet.title.to_string();
    server_response.playlist.thumbnails = data.items[0].snippet.thumbnails.clone();

    Ok(())
}
