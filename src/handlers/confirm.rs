use anyhow::Result;

use crate::{app::App, db::books::Book, event::Key};

use super::common_key_events;

fn handle_esc(app: &mut App) {
    app.selected_book_index = None;
    app.reset_navigation_stack();
}

fn handle_enter(app: &mut App) -> Result<()> {
    if app.confirm == false {
        handle_esc(app);
        return Ok(());
    }

    let book_index = match app.selected_book_index {
        Some(i) => i,
        None => 0,
    };

    let book = &app.books[book_index];
    Book::remove_by_id(&app.db.db_file_path, &book.id)?;

    app.books.remove(book_index);
    app.selected_book_index = None;

    handle_esc(app);
    Ok(())
}

pub fn handler(key: Key, app: &mut App) -> Result<()> {
    match key {
        Key::Esc => {
            handle_esc(app);
        }
        k if common_key_events::any_side(k) => app.confirm = !app.confirm,
        Key::Enter => {
            handle_enter(app)?;
        }
        _ => {}
    }
    Ok(())
}
