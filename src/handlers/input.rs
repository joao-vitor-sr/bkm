use anyhow::Result;

use crate::{
    app::{ActiveBlock, App},
    event::Key,
};

pub fn handler(key: Key, app: &mut App) -> Result<()> {
    match key {
        Key::Enter => {
            app.books.push(app.input.drain(..).collect());
            app.insert_books()?;
        }
        Key::Char(c) => {
            app.input.push(c);
        }
        Key::Backspace => {
            app.input.pop();
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Home), Some(ActiveBlock::Home));
        }
        _ => {}
    }
    Ok(())
}
