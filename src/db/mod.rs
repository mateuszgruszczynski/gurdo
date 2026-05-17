pub mod schema;
pub mod queries;

pub use schema::init_db;

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

pub fn open(db_path: &Path) -> Result<Connection> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    init_db(&conn)?;
    Ok(conn)
}
