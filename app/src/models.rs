use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ImportResponse {
    pub accepted: bool,
    pub song: Option<SongMetadata>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SongMetadata {
    pub source_url: String,
    pub title: Option<String>,
    pub lyrics: Option<String>,
    pub audio_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}
