use anyhow::Result;

use crate::{
    app::App,
    db::books::{Book, BookInputs},
    event::Key,
};

fn process_input(app: &mut App) -> Result<()> {
    if let Some(book) = &mut app.book {
        book.insert_book(&app.db.db_file_path)?;

        app.books.push(book.clone());

        app.book = None;
        app.reset_navigation_stack();

        app.selected_input = BookInputs::Name;
    }
    Ok(())
}

pub fn handler(key: Key, app: &mut App) -> Result<()> {
    match key {
        Key::Tab => {
            app.selected_input = match app.selected_input {
                BookInputs::Name => BookInputs::Author,
                BookInputs::Author => BookInputs::Date,
                BookInputs::Date => BookInputs::Name,
            };
        }

        Key::Esc => {
            app.reset_navigation_stack();
        }

        Key::Enter => {
            process_input(app)?;
        }

        Key::Char(c) => {
            match app.book {
                None => {
                    app.book = Some(Book::new());
                }
                _ => {}
            };

            if let Some(book) = &mut app.book {
                match app.selected_input {
                    BookInputs::Name => {
                        book.name.push(c);
                    }
                    BookInputs::Date => {
                        book.date.push(c);
                    }
                    BookInputs::Author => {
                        book.author.push(c);
                    }
                }
            }
        }

        Key::Backspace | Key::Ctrl('h') => {
            if let Some(book) = &mut app.book {
                match app.selected_input {
                    BookInputs::Name => {
                        book.name.pop();
                    }
                    BookInputs::Author => {
                        book.author.pop();
                    }
                    BookInputs::Date => {
                        book.date.pop();
                    }
                };
            }
        }

        _ => {}
    }
    Ok(())
}
