use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct PlaylistDetailsBody {
    pub list: String,
}
