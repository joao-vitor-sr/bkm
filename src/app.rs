use crate::{
    db::{books::Book, Db},
    ui::list::StatefulList,
};
use anyhow::Result;
use std::{path::PathBuf, time::Duration};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Input,
    Books,
    Home,
}

#[derive(Debug)]
pub struct Route {
    pub block: ActiveBlock,
}

const DEFAULT_ROUTE: Route = Route {
    block: ActiveBlock::Home,
};

#[derive(Debug)]
pub struct App<'a> {
    navigation_stack: Vec<Route>,
    pub title: &'a str,
    pub should_quit: bool,
    pub tick_rate_milliseconds: u64,
    pub db: Db,
    pub books_list: StatefulList<(String, String)>,

    // current value of the input box
    pub input: String,

    // history of the recorded books
    pub books: Vec<String>,
}

impl<'a> App<'a> {
    pub fn get_current_route(&self) -> &Route {
        // if for some reason there is no route return the default
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let mut current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.block = active_block;
        }
    }

    pub fn insert_books(&mut self) -> Result<()> {
        for book in &self.books {
            let id = Book::insert_book(&self.db.db_file_path, book)?;
            self.books_list.items.push((book.clone(), id));
        }
        self.books = Vec::new();
        Ok(())
    }

    pub fn new(
        title: &'a str,
        tick_rate_milliseconds: Option<u64>,
        custom_db_path: Option<PathBuf>,
    ) -> Result<App<'a>> {
        let db = Db::new(custom_db_path)?;
        db.set_up_tables()?;

        let books = Book::return_stateful_books(&db.db_file_path)?;
        Ok(App {
            title,
            should_quit: false,
            navigation_stack: vec![DEFAULT_ROUTE],
            tick_rate_milliseconds: match tick_rate_milliseconds {
                Some(v) => v,
                None => 250,
            },
            db,
            books_list: books,
            books: Vec::new(),
            input: String::new(),
        })
    }
}
