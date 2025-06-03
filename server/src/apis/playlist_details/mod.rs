mod playlist;
mod playlist_items;
mod videos;

use axum::{Json, http::StatusCode};
use playlist::playlist;
use playlist_items::playlist_items;
use serde::Serialize;
use videos::videos;

use crate::{entities::response::ServerResponse, utils::error::ApiError};

use super::entities::PlaylistDetailsBody;

#[derive(Serialize)]
pub struct PlaylistDetailsResponse {
    success: bool,
    message: String,
    data: ServerResponse,
}

pub async fn playlist_details(
    Json(body): Json<PlaylistDetailsBody>,
) -> Result<Json<PlaylistDetailsResponse>, ApiError> {
    let Ok(api_key) = std::env::var("YOUTUBE_API_KEY") else {
        let error = ApiError::new(
            vec![String::from("Youtube API key not available")],
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return Err(error);
    };

    let mut server_response = ServerResponse::default();

    let video_ids = playlist_items(&api_key, &body.list, &mut server_response).await?;
    playlist(&api_key, &body.list, &mut server_response).await?;
    videos(&api_key, video_ids, &mut server_response).await?;

    let response = Json(PlaylistDetailsResponse {
        success: true,
        message: String::from("Playlist details ⭐"),
        data: server_response,
    });

    Ok(response)
}
