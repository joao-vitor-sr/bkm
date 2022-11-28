use crate::ui::list::StatefulList;
use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug)]
pub struct Book<'a> {
    pub id: String,
    pub name: String,
    pub db_path: Option<&'a PathBuf>,
}

impl<'a> Book<'a> {
    pub fn insert_book(db_path: &PathBuf, name: &String) -> Result<String> {
        let conn = Connection::open(db_path)?;

        let id = Uuid::new_v4().hyphenated().to_string();

        conn.execute(
            "INSERT INTO books (name, id) VALUES (?1, ?2)",
            (name, id.clone()),
        )?;
        Ok(id)
    }

    pub fn return_stateful_books(db_path: &PathBuf) -> Result<StatefulList<(String, String)>> {
        let books = Book::return_books(db_path)?;
        let books: Vec<(String, String)> = books
            .iter()
            .map(|f| (f.name.clone(), f.id.clone()))
            .collect();
        Ok(StatefulList::with_items(books))
    }

    pub fn return_books(db_path: &PathBuf) -> Result<Vec<Book>> {
        let conn = Connection::open(db_path)?;

        let mut stmt = conn.prepare("SELECT id, name FROM books")?;
        let book_iter = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                name: row.get(1)?,
                db_path: None,
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
