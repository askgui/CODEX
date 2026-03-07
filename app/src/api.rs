use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json,
    Router,
};
use reqwest::Client;
use tracing::info;

use crate::{
    importer::{import_from_url, ImportError},
    models::{HealthResponse, ImportRequest, ImportResponse},
};

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/import", post(import))
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "music-importer-api",
    })
}

async fn import(
    State(state): State<AppState>,
    Json(payload): Json<ImportRequest>,
) -> impl IntoResponse {
    let source_url = payload.url.trim();
    info!(url = %source_url, "Recebida solicitação de importação");

    if source_url.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ImportResponse {
                accepted: false,
                song: None,
                message: "Campo url é obrigatório".to_string(),
            }),
        )
            .into_response();
    }

    match import_from_url(&state.http_client, source_url).await {
        Ok(song) => (
            StatusCode::OK,
            Json(ImportResponse {
                accepted: true,
                song: Some(song),
                message: "Importação iniciada com sucesso (MVP)".to_string(),
            }),
        )
            .into_response(),
        Err(err) => {
            let (status, message) = match err {
                ImportError::InvalidUrl => (StatusCode::BAD_REQUEST, err.to_string()),
                ImportError::UnsupportedHost => (StatusCode::BAD_REQUEST, err.to_string()),
                ImportError::Download(_) => (StatusCode::BAD_GATEWAY, err.to_string()),
            };

            (
                status,
                Json(ImportResponse {
                    accepted: false,
                    song: None,
                    message,
                }),
            )
                .into_response()
        }
    }
}
