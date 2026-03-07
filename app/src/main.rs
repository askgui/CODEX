mod api;
mod db;
mod importer;
mod models;

use std::net::SocketAddr;

use reqwest::Client;
use tokio::net::TcpListener;
use tracing::info;

use crate::api::{router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .compact()
        .init();

    let db_path = "music_importer.db".to_string();
    db::init(&db_path).expect("falha ao inicializar banco SQLite");

    let state = AppState {
        http_client: Client::builder()
            .user_agent("music-importer/0.1")
            .build()
            .expect("falha ao criar cliente HTTP"),
        db_path,
    };

    let app = router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(addr)
        .await
        .expect("falha ao abrir porta 7878");

    info!("API local disponível em http://{addr}");
    axum::serve(listener, app)
        .await
        .expect("erro no servidor HTTP");
}
