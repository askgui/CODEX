use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    routing::post,
    Json, Router,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::info;

use crate::{
    db,
    importer::{import_from_url, ImportError},
    models::{
        ApiErrorResponse, HealthResponse, ImportItem, ImportRequest, ImportResponse,
        ImportsListResponse,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub db_path: String,
}

#[derive(Debug, Deserialize)]
struct ImportsQuery {
    limit: Option<usize>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/import", post(import))
        .route("/imports", get(list_imports))
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
                import_id: None,
                song: None,
                message: "Campo url é obrigatório".to_string(),
            }),
        )
            .into_response();
    }

    match import_from_url(&state.http_client, source_url).await {
        Ok(song) => {
            let import_id = match db::save_import(&state.db_path, &song) {
                Ok(id) => id,
                Err(err) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ImportResponse {
                            accepted: false,
                            import_id: None,
                            song: None,
                            message: format!("Falha ao persistir importação: {err}"),
                        }),
                    )
                        .into_response()
                }
            };

            (
                StatusCode::OK,
                Json(ImportResponse {
                    accepted: true,
                    import_id: Some(import_id),
                    song: Some(song),
                    message: "Importação iniciada e persistida com sucesso".to_string(),
                }),
            )
                .into_response()
        }
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
                    import_id: None,
                    song: None,
                    message,
                }),
            )
                .into_response()
        }
    }
}

async fn list_imports(
    State(state): State<AppState>,
    Query(query): Query<ImportsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);

    match db::list_imports(&state.db_path, limit) {
        Ok(rows) => {
            let items = rows
                .into_iter()
                .map(|r| ImportItem {
                    id: r.id,
                    source_url: r.source_url,
                    title: r.title,
                    audio_url: r.audio_url,
                    created_at: r.created_at,
                })
                .collect();

            (StatusCode::OK, Json(ImportsListResponse { items })).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorResponse {
                error: format!("Falha ao listar importações: {err}"),
            }),
        )
            .into_response(),
    }
}
