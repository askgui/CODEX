use rusqlite::{params, Connection};

use crate::models::SongMetadata;

#[derive(Debug, Clone)]
pub struct ImportRecord {
    pub id: i64,
    pub source_url: String,
    pub title: Option<String>,
    pub lyrics: Option<String>,
    pub audio_url: Option<String>,
    pub local_audio_path: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct ImportStats {
    pub total: i64,
    pub parsed: i64,
    pub downloaded: i64,
    pub failed: i64,
}

fn ensure_column(
    conn: &Connection,
    table: &str,
    column: &str,
    ddl: &str,
) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let cols = stmt.query_map([], |row| row.get::<_, String>(1))?;

    let exists = cols.flatten().any(|c| c == column);
    if !exists {
        conn.execute_batch(&format!("ALTER TABLE {table} ADD COLUMN {column} {ddl};"))?;
    }

    Ok(())
}

pub fn init(db_path: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS imports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_url TEXT NOT NULL UNIQUE,
            title TEXT,
            lyrics TEXT,
            audio_url TEXT,
            local_audio_path TEXT,
            status TEXT NOT NULL DEFAULT 'parsed',
            error_message TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE INDEX IF NOT EXISTS idx_imports_created_at ON imports(created_at DESC);
        ",
    )?;

    ensure_column(&conn, "imports", "local_audio_path", "TEXT")?;
    ensure_column(&conn, "imports", "status", "TEXT NOT NULL DEFAULT 'parsed'")?;
    ensure_column(&conn, "imports", "error_message", "TEXT")?;

    Ok(())
}

pub fn upsert_import(db_path: &str, song: &SongMetadata) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "
        INSERT INTO imports (source_url, title, lyrics, audio_url, local_audio_path, status, error_message)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ON CONFLICT(source_url) DO UPDATE SET
            title=excluded.title,
            lyrics=excluded.lyrics,
            audio_url=excluded.audio_url,
            local_audio_path=excluded.local_audio_path,
            status=excluded.status,
            error_message=excluded.error_message
        ",
        params![
            song.source_url,
            song.title,
            song.lyrics,
            song.audio_url,
            song.local_audio_path,
            if song.local_audio_path.is_some() {
                "downloaded"
            } else {
                "parsed"
            },
            Option::<String>::None,
        ],
    )?;

    let id: i64 = conn.query_row(
        "SELECT id FROM imports WHERE source_url = ?1",
        params![song.source_url],
        |row| row.get(0),
    )?;

    Ok(id)
}

pub fn mark_import_error(
    db_path: &str,
    source_url: &str,
    error_message: &str,
) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "
        INSERT INTO imports (source_url, status, error_message)
        VALUES (?1, 'failed', ?2)
        ON CONFLICT(source_url) DO UPDATE SET
            status='failed',
            error_message=excluded.error_message
        ",
        params![source_url, error_message],
    )?;

    Ok(())
}

pub fn list_imports(db_path: &str, limit: usize) -> Result<Vec<ImportRecord>, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "
        SELECT id, source_url, title, lyrics, audio_url, local_audio_path, status, error_message, created_at
        FROM imports
        ORDER BY id DESC
        LIMIT ?1
        ",
    )?;

    let rows = stmt.query_map(params![limit as i64], |row| {
        Ok(ImportRecord {
            id: row.get(0)?,
            source_url: row.get(1)?,
            title: row.get(2)?,
            lyrics: row.get(3)?,
            audio_url: row.get(4)?,
            local_audio_path: row.get(5)?,
            status: row.get(6)?,
            error_message: row.get(7)?,
            created_at: row.get(8)?,
        })
    })?;

    rows.collect()
}

pub fn get_import_by_id(db_path: &str, id: i64) -> Result<Option<ImportRecord>, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "
        SELECT id, source_url, title, lyrics, audio_url, local_audio_path, status, error_message, created_at
        FROM imports
        WHERE id = ?1
        ",
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(ImportRecord {
            id: row.get(0)?,
            source_url: row.get(1)?,
            title: row.get(2)?,
            lyrics: row.get(3)?,
            audio_url: row.get(4)?,
            local_audio_path: row.get(5)?,
            status: row.get(6)?,
            error_message: row.get(7)?,
            created_at: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn delete_import_by_id(db_path: &str, id: i64) -> Result<bool, rusqlite::Error> {
    let conn = Connection::open(db_path)?;
    let changed = conn.execute("DELETE FROM imports WHERE id = ?1", params![id])?;
    Ok(changed > 0)
}

pub fn fetch_stats(db_path: &str) -> Result<ImportStats, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM imports", [], |row| row.get(0))?;
    let parsed: i64 = conn.query_row(
        "SELECT COUNT(*) FROM imports WHERE status = 'parsed'",
        [],
        |row| row.get(0),
    )?;
    let downloaded: i64 = conn.query_row(
        "SELECT COUNT(*) FROM imports WHERE status = 'downloaded'",
        [],
        |row| row.get(0),
    )?;
    let failed: i64 = conn.query_row(
        "SELECT COUNT(*) FROM imports WHERE status = 'failed'",
        [],
        |row| row.get(0),
    )?;

    Ok(ImportStats {
        total,
        parsed,
        downloaded,
        failed,
    })
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        delete_import_by_id, fetch_stats, get_import_by_id, init, list_imports, mark_import_error,
        upsert_import,
    };
    use crate::models::SongMetadata;

    #[test]
    fn init_upsert_and_list_imports() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let db_path = std::env::temp_dir().join(format!("music-importer-test-{nonce}.db"));
        let db_path = db_path.to_string_lossy().to_string();

        init(&db_path).expect("init db");

        let song = SongMetadata {
            source_url: "https://suno.com/song/abc".to_string(),
            title: Some("Teste".to_string()),
            lyrics: None,
            audio_url: Some("https://cdn.suno.com/abc.mp3".to_string()),
            local_audio_path: Some("downloads/abc.mp3".to_string()),
        };

        let id = upsert_import(&db_path, &song).expect("upsert import");
        assert!(id > 0);

        let rows = list_imports(&db_path, 10).expect("list imports");
        assert!(!rows.is_empty());
        assert_eq!(rows[0].source_url, "https://suno.com/song/abc");
        assert_eq!(rows[0].status, "downloaded");

        let by_id = get_import_by_id(&db_path, id)
            .expect("get by id")
            .expect("exists");
        assert_eq!(by_id.id, id);

        assert!(delete_import_by_id(&db_path, id).expect("delete by id"));
        assert!(get_import_by_id(&db_path, id)
            .expect("check by id")
            .is_none());
    }

    #[test]
    fn mark_error_upserts_row() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let db_path = std::env::temp_dir().join(format!("music-importer-test-{nonce}.db"));
        let db_path = db_path.to_string_lossy().to_string();

        init(&db_path).expect("init db");
        mark_import_error(&db_path, "https://suno.com/song/fail", "erro").expect("mark error");

        let rows = list_imports(&db_path, 10).expect("list");
        assert_eq!(rows[0].status, "failed");
        assert_eq!(rows[0].error_message.as_deref(), Some("erro"));

        let stats = fetch_stats(&db_path).expect("stats");
        assert_eq!(stats.total, 1);
        assert_eq!(stats.failed, 1);
    }
}
