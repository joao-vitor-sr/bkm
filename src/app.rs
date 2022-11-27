use crate::{db::{Db, books::Book}, ui::list::StatefulList};
use anyhow::Result;
use std::{path::PathBuf, time::Duration};

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub input_timeout: Duration,
    pub db: Db,
    pub books_list: StatefulList<(String, usize)>,
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
        })
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
