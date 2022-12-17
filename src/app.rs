use crate::db::{
    books::{Book, BookInputs},
    Db,
};
use anyhow::Result;
use std::path::PathBuf;
use tui::style::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Input,
    Books,
    Confirm,
}

#[derive(Debug)]
pub struct Route {
    pub block: ActiveBlock,
}

pub const DEFAULT_ROUTE: Route = Route {
    block: ActiveBlock::Books,
};

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub active: Color,
    pub inactive: Color,
    pub text: Color,
}

#[derive(Debug)]
pub struct App {
    navigation_stack: Vec<Route>,
    pub should_quit: bool,
    pub tick_rate_milliseconds: u64,
    pub db: Db,
    pub books: Vec<Book>,

    pub book: Option<Book>,

    pub selected_book_id: Option<String>,
    pub selected_book_index: Option<usize>,
    pub theme: Theme,

    // this options is only used for the Confirm route
    pub confirm: bool,
    pub selected_input: BookInputs,
}

impl App {
    pub fn reset_navigation_stack(&mut self) {
        self.navigation_stack = vec![DEFAULT_ROUTE];
    }

    pub fn get_current_route(&self) -> &Route {
        // if for some reason there is no route return the default
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_route_state(&mut self, active_block: Option<ActiveBlock>) {
        let mut current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.block = active_block;
        }
    }
    pub fn new(
        tick_rate_milliseconds: Option<u64>,
        custom_db_path: Option<PathBuf>,
    ) -> Result<App> {
        let db = Db::new(custom_db_path)?;
        db.set_up_tables()?;

        let books = Book::return_books(&db.db_file_path)?;
        Ok(App {
            book: None,
            should_quit: false,
            navigation_stack: vec![DEFAULT_ROUTE],
            tick_rate_milliseconds: match tick_rate_milliseconds {
                Some(v) => v,
                None => 250,
            },
            db,
            books,
            selected_book_index: None,
            selected_book_id: None,
            confirm: false,
            theme: Default::default(),
            selected_input: BookInputs::Name,
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            active: Color::Cyan,
            inactive: Color::Gray,
            text: Color::White,
        }
    }
}
