use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug)]
pub struct Book {
    pub id: String,
    pub name: String,
}

impl Book {
    pub fn remove_by_id(db_path: &PathBuf, id: &String) -> Result<()> {
        let conn = Connection::open(db_path)?;

        conn.execute("DELETE FROM books WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn insert_book(db_path: &PathBuf, name: String) -> Result<(String, String)> {
        let conn = Connection::open(db_path)?;

        let id = Uuid::new_v4().hyphenated().to_string();

        conn.execute(
            "INSERT INTO books (name, id) VALUES (?1, ?2)",
            (name.clone(), id.clone()),
        )?;
        Ok((id, name))
    }
    pub fn return_books(db_path: &PathBuf) -> Result<Vec<Book>> {
        let conn = Connection::open(db_path)?;

        let mut stmt = conn.prepare("SELECT id, name FROM books")?;
        let book_iter = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut books = Vec::new();
        for book in book_iter {
            books.push(book?);
        }

        Ok(books)
    }

    pub fn create_table(db_path: &PathBuf) -> Result<()> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                        id TEXT PRIMARY KEY,
                        name TEXT NOT NULL)",
            (),
        )?;

        Ok(())
    }
}
