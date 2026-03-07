use std::path::{Path as FsPath, PathBuf};

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
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
    offset: Option<usize>,
    status: Option<String>,
    q: Option<String>,
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
        .route("/imports/:id/audio", get(stream_import_audio))
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
    let offset = query.offset.unwrap_or(0);
    let status_filter = query
        .status
        .as_deref()
        .filter(|s| matches!(*s, "parsed" | "downloaded" | "failed"));

    let search_filter = query.q.as_deref().map(str::trim).filter(|s| !s.is_empty());

    let total = match db::count_imports(&state.db_path, status_filter, search_filter) {
        Ok(total) => total,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    error: format!("Falha ao contar importações: {err}"),
                }),
            )
                .into_response();
        }
    };

    match db::list_imports(&state.db_path, limit, offset, status_filter, search_filter) {
        Ok(rows) => {
            let items: Vec<ImportItem> = rows
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

            let has_more = (offset + items.len()) < (total as usize);

            (
                StatusCode::OK,
                Json(ImportsListResponse {
                    items,
                    total,
                    limit,
                    offset,
                    has_more,
                }),
            )
                .into_response()
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

async fn stream_import_audio(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Response, (StatusCode, Json<ApiErrorResponse>)> {
    let import = db::get_import_by_id(&state.db_path, id)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    error: format!("Falha ao consultar importação: {err}"),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiErrorResponse {
                    error: format!("Importação id={id} não encontrada"),
                }),
            )
        })?;

    let audio_path = import.local_audio_path.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiErrorResponse {
                error: "Áudio local não disponível para esta importação".to_string(),
            }),
        )
    })?;

    let path = FsPath::new(&audio_path);
    if !path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiErrorResponse {
                error: "Arquivo de áudio não encontrado no disco".to_string(),
            }),
        ));
    }

    let bytes = tokio::fs::read(path).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorResponse {
                error: format!("Falha ao ler arquivo de áudio: {err}"),
            }),
        )
    })?;

    let mut response = Response::new(Body::from(bytes));
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));

    Ok(response)
}
