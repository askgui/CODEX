use std::path::PathBuf;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

use crate::{
    db,
    importer::{import_from_url, maybe_download_audio, ImportError},
    models::{
        ActionResponse, ApiErrorResponse, HealthResponse, ImportItem, ImportRequest,
        ImportResponse, ImportsListResponse, StatsResponse,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub db_path: String,
    pub downloads_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ImportsQuery {
    limit: Option<usize>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats))
        .route("/import", post(import))
        .route("/imports", get(list_imports))
        .route(
            "/imports/:id",
            get(get_import_by_id).delete(delete_import_by_id),
        )
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "music-importer-api",
    })
}

async fn stats(State(state): State<AppState>) -> impl IntoResponse {
    match db::fetch_stats(&state.db_path) {
        Ok(s) => (
            StatusCode::OK,
            Json(StatsResponse {
                total: s.total,
                parsed: s.parsed,
                downloaded: s.downloaded,
                failed: s.failed,
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorResponse {
                error: format!("Falha ao calcular estatísticas: {err}"),
            }),
        )
            .into_response(),
    }
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
        Ok(mut song) => {
            match maybe_download_audio(&state.http_client, &song, &state.downloads_dir).await {
                Ok(local_file) => {
                    song.local_audio_path = local_file;
                }
                Err(err) => {
                    warn!(url = %source_url, error = %err, "Falha no download do áudio, continuando com metadados");
                }
            }

            let import_id = match db::upsert_import(&state.db_path, &song) {
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
                    message: "Importação processada e persistida com sucesso".to_string(),
                }),
            )
                .into_response()
        }
        Err(err) => {
            let _ = db::mark_import_error(&state.db_path, source_url, &err.to_string());

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
                    local_audio_path: r.local_audio_path,
                    status: r.status,
                    error_message: r.error_message,
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

async fn get_import_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match db::get_import_by_id(&state.db_path, id) {
        Ok(Some(item)) => (
            StatusCode::OK,
            Json(ImportItem {
                id: item.id,
                source_url: item.source_url,
                title: item.title,
                audio_url: item.audio_url,
                local_audio_path: item.local_audio_path,
                status: item.status,
                error_message: item.error_message,
                created_at: item.created_at,
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiErrorResponse {
                error: format!("Importação id={id} não encontrada"),
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorResponse {
                error: format!("Falha ao consultar importação: {err}"),
            }),
        )
            .into_response(),
    }
}

async fn delete_import_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match db::delete_import_by_id(&state.db_path, id) {
        Ok(true) => (
            StatusCode::OK,
            Json(ActionResponse {
                ok: true,
                message: format!("Importação id={id} removida"),
            }),
        )
            .into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ActionResponse {
                ok: false,
                message: format!("Importação id={id} não encontrada"),
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ActionResponse {
                ok: false,
                message: format!("Falha ao remover importação: {err}"),
            }),
        )
            .into_response(),
    }
}
