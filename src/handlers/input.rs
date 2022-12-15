use anyhow::Result;

use crate::{app::App, db::books::BookInputs, event::Key};

fn process_input(app: &mut App) -> Result<()> {
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

        Key::Char(c) => match app.selected_input {
            BookInputs::Name => {
                app.book.name.push(c);
            }
            BookInputs::Date => {
                app.book.date.push(c);
            }
            BookInputs::Author => {
                app.book.author.push(c);
            }
        },

        Key::Backspace | Key::Ctrl('h') => {
            match app.selected_input {
                BookInputs::Name => {
                    app.book.name.pop();
                }
                BookInputs::Author => {
                    app.book.author.pop();
                }
                BookInputs::Date => {
                    app.book.date.pop();
                }
            };
        }

        _ => {}
    }
    Ok(())
}
