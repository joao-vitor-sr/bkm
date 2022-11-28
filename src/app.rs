use crate::{
    db::{books::Book, Db},
    ui::list::StatefulList,
};
use anyhow::Result;
use std::{path::PathBuf, time::Duration};

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub input_timeout: Duration,
    pub db: Db,
    pub books_list: StatefulList<(String, usize)>,
    pub add_book_mode: InputMode,

    // current value of the input box
    pub input: String,

    // history of the recorded books
    pub books: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        input_timeout: Duration,
        custom_db_path: Option<PathBuf>,
    ) -> Result<App<'a>> {
        let db = Db::new(custom_db_path)?;
        db.set_up_tables()?;

        let books = Book::return_stateful_books(&db.db_file_path)?;
        Ok(App {
            title,
            should_quit: false,
            input_timeout,
            db,
            books_list: books,
            add_book_mode: InputMode::Normal,
            books: Vec::new(),
            input: String::new(),
        })
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'a' => self.add_book_mode = InputMode::Editing,
            _ => {}
        }
    }
}
