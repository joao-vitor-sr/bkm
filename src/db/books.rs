use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub name: String,
}

impl Book {
    pub fn create_table(db_path: &PathBuf) -> Result<()> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL)",
            (),
        )?;

        Ok(())
    }
}
