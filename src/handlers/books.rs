use unicode_width::UnicodeWidthStr;

use super::common_key_events;
use crate::{
    app::{ActiveBlock, App},
    event::Key,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.selected_book_index = None;
            app.reset_navigation_stack();
        }
        Key::Char('a') => {
            app.set_current_route_state(Some(ActiveBlock::Input));
        }
        k if common_key_events::down_event(k) => {
            let next_index =
                common_key_events::on_down_press_handler(&app.books, app.selected_book_index);
            app.selected_book_index = Some(next_index);
        }
        k if common_key_events::up_event(k) => {
            let next_index =
                common_key_events::on_up_press_handler(&app.books, app.selected_book_index);
            app.selected_book_index = Some(next_index);
        }
        Key::Char('d') => {
            app.set_current_route_state(Some(ActiveBlock::Confirm));
        }
        Key::Char('e') => {
            let book_index = match app.selected_book_index {
                Some(i) => i,
                None => 0,
            };

            let book = &app.books[book_index];

            app.input = book.name.chars().collect();
            app.input_idx = app.input.len();
            app.input_cursor_position = UnicodeWidthStr::width(book.name.as_str())
                .try_into()
                .unwrap();

            app.selected_book_id = Some(book.id.clone());
            app.set_current_route_state(Some(ActiveBlock::Input));
        }
        _ => {}
    }
}
