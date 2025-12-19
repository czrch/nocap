use rusqlite::{Connection, Result};

const SCHEMA_VERSION: i32 = 1;

#[allow(dead_code)]
pub fn init_schema(conn: &Connection) -> Result<()> {
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Check if schema is already initialized
    let version: Result<i32> = conn.query_row(
        "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get(0),
    );

    if let Ok(v) = version {
        if v >= SCHEMA_VERSION {
            return Ok(());
        }
    }

    // Create schema version table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Media files discovered during scanning
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            filename TEXT NOT NULL,
            extension TEXT NOT NULL,
            media_type TEXT NOT NULL,
            file_size INTEGER,
            file_hash TEXT,
            created_at TEXT,
            modified_at TEXT,
            scanned_at TEXT NOT NULL DEFAULT (datetime('now')),
            thumbnail_path TEXT
        )",
        [],
    )?;

    // Embedded metadata extracted from files
    conn.execute(
        "CREATE TABLE IF NOT EXISTS embedded_metadata (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            media_file_id INTEGER NOT NULL UNIQUE,
            width INTEGER,
            height INTEGER,
            duration_seconds REAL,
            bit_rate INTEGER,
            codec TEXT,
            camera_make TEXT,
            camera_model TEXT,
            lens TEXT,
            focal_length TEXT,
            aperture TEXT,
            shutter_speed TEXT,
            iso INTEGER,
            taken_at TEXT,
            gps_latitude REAL,
            gps_longitude REAL,
            title TEXT,
            artist TEXT,
            album TEXT,
            year INTEGER,
            genre TEXT,
            track_number INTEGER,
            raw_metadata TEXT,
            FOREIGN KEY (media_file_id) REFERENCES media_files(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // User-defined tags
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT
        )",
        [],
    )?;

    // Many-to-many relationship between media and tags
    conn.execute(
        "CREATE TABLE IF NOT EXISTS media_tags (
            media_file_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (media_file_id, tag_id),
            FOREIGN KEY (media_file_id) REFERENCES media_files(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // User annotations for media files
    conn.execute(
        "CREATE TABLE IF NOT EXISTS annotations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            media_file_id INTEGER NOT NULL UNIQUE,
            rating INTEGER CHECK (rating >= 0 AND rating <= 5),
            comment TEXT,
            notes TEXT,
            favorite INTEGER DEFAULT 0,
            ai_description TEXT,
            ai_tags TEXT,
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (media_file_id) REFERENCES media_files(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Settings for AI providers
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider TEXT NOT NULL,
            api_key TEXT,
            base_url TEXT,
            model TEXT,
            is_active INTEGER DEFAULT 0
        )",
        [],
    )?;

    // Create indexes for common queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_media_type ON media_files(media_type)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_media_path ON media_files(path)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_annotations_rating ON annotations(rating)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_annotations_favorite ON annotations(favorite)",
        [],
    )?;

    // Record schema version
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        [SCHEMA_VERSION],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_init_schema() {
        let conn = Connection::open_in_memory().unwrap();
        init_schema(&conn).unwrap();

        // Verify all tables exist
        let tables = vec![
            "schema_version",
            "media_files",
            "embedded_metadata",
            "tags",
            "media_tags",
            "annotations",
            "ai_settings",
        ];

        for table in tables {
            let count: i64 = conn
                .query_row(
                    &format!(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'",
                        table
                    ),
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "Table {} should exist", table);
        }

        // Verify schema version
        let version: i32 = conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn test_schema_idempotence() {
        let conn = Connection::open_in_memory().unwrap();

        // Initialize twice
        init_schema(&conn).unwrap();
        init_schema(&conn).unwrap();

        // Should still have only one version record
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}
