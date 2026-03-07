use rusqlite::{params, Connection};

use crate::models::SongMetadata;

#[derive(Debug, Clone)]
pub struct ImportRecord {
    pub id: i64,
    pub source_url: String,
    pub title: Option<String>,
    pub lyrics: Option<String>,
    pub audio_url: Option<String>,
    pub created_at: String,
}

pub fn init(db_path: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS imports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_url TEXT NOT NULL,
            title TEXT,
            lyrics TEXT,
            audio_url TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE INDEX IF NOT EXISTS idx_imports_created_at ON imports(created_at DESC);
        ",
    )?;

    Ok(())
}

pub fn save_import(db_path: &str, song: &SongMetadata) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "
        INSERT INTO imports (source_url, title, lyrics, audio_url)
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![song.source_url, song.title, song.lyrics, song.audio_url],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn list_imports(db_path: &str, limit: usize) -> Result<Vec<ImportRecord>, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "
        SELECT id, source_url, title, lyrics, audio_url, created_at
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
            created_at: row.get(5)?,
        })
    })?;

    rows.collect()
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{init, list_imports, save_import};
    use crate::models::SongMetadata;

    #[test]
    fn init_save_and_list_imports() {
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
        };

        let id = save_import(&db_path, &song).expect("save import");
        assert!(id > 0);

        let rows = list_imports(&db_path, 10).expect("list imports");
        assert!(!rows.is_empty());
        assert_eq!(rows[0].source_url, "https://suno.com/song/abc");
    }
}
