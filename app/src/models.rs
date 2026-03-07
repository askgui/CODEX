use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ImportResponse {
    pub accepted: bool,
    pub import_id: Option<i64>,
    pub song: Option<SongMetadata>,
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SongMetadata {
    pub source_url: String,
    pub title: Option<String>,
    pub lyrics: Option<String>,
    pub audio_url: Option<String>,
    pub local_audio_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportItem {
    pub id: i64,
    pub source_url: String,
    pub title: Option<String>,
    pub audio_url: Option<String>,
    pub local_audio_path: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ImportsListResponse {
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}
