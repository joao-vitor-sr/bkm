use super::common_key_events;
use crate::{
    app::{ActiveBlock, App},
    event::Key,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.selected_book_index = None;
            app.set_current_route_state(Some(ActiveBlock::Home));
        }
        Key::Down => {
            let next_index =
                common_key_events::on_down_press_handler(&app.books, app.selected_book_index);
            app.selected_book_index = Some(next_index);
        }
        Key::Up => {
            let next_index =
                common_key_events::on_up_press_handler(&app.books, app.selected_book_index);
            app.selected_book_index = Some(next_index);
        }
        _ => {}
    }
}
