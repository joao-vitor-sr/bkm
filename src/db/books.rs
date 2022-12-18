use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub date: String,
}

#[derive(Debug)]
pub enum BookInputs {
    Name,
    Author,
    Date,
}

impl Book {
    pub fn remove_by_id(db_path: &PathBuf, id: &String) -> Result<()> {
        let conn = Connection::open(db_path)?;

        conn.execute("DELETE FROM books WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn save_record(&mut self, db_path: &PathBuf) -> Result<()> {
        if self.id == "" {
            self.insert_book(db_path)?;
        } else {
            self.update_book(db_path)?;
        }

        Ok(())
    }

    pub fn insert_book(&mut self, db_path: &PathBuf) -> Result<()> {
        let conn = Connection::open(db_path)?;

        self.id = Uuid::new_v4().hyphenated().to_string();

        conn.execute(
            "INSERT INTO books (name, author, date, id) VALUES (?1, ?2, ?3, ?4)",
            (
                self.name.as_str(),
                self.author.as_str(),
                self.date.as_str(),
                self.id.as_str(),
            ),
        )?;
        Ok(())
    }
    pub fn return_books(db_path: &PathBuf) -> Result<Vec<Book>> {
        let conn = Connection::open(db_path)?;

        let mut stmt = conn.prepare("SELECT id, name, author, date FROM books")?;
        let book_iter = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                date: row.get(3)?,
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
                        name TEXT NOT NULL,
                        author TEXT,
                        date TEXT
)",
            (),
        )?;

        Ok(())
    }

    pub fn update_book(&mut self, db_path: &PathBuf) -> Result<()> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "UPDATE books SET name = ?1 WHERE id = ?2",
            params![self.name.as_str(), self.id.as_str()],
        )?;

        Ok(())
    }

    pub fn new() -> Book {
        Book {
            id: String::new(),
            name: String::new(),
            date: String::new(),
            author: String::new(),
        }
    }
}
