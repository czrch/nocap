pub mod schema;

use rusqlite::{Connection, Result};
use std::path::Path;

/// Initialize or open a database connection for a project
pub fn open_project_db(project_path: &Path) -> Result<Connection> {
    let nocap_dir = project_path.join(".nocap");
    std::fs::create_dir_all(&nocap_dir)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    let db_path = nocap_dir.join("metadata.db");
    let conn = Connection::open(db_path)?;

    // Initialize schema
    schema::init_schema(&conn)?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_open_project_db() {
        let temp_dir = env::temp_dir().join("nocap_test");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let conn = open_project_db(&temp_dir).unwrap();

        // Verify schema was created
        let result: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(result > 0, "Schema should create tables");

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
