use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

use crate::ui::list::StatefulList;

#[derive(Debug)]
pub struct Book {
    pub id: usize,
    pub name: String,
}

impl Book {
    pub fn return_stateful_books(db_path: &PathBuf) -> Result<StatefulList<(String, usize)>> {
        let books = Book::return_books(db_path)?;
        let books: Vec<(String, usize)> = books.iter().map(|f| (f.name.clone(), f.id)).collect();
        Ok(StatefulList::with_items(books))
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
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL)",
            (),
        )?;

        Ok(())
    }
}
