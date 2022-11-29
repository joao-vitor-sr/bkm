use crate::{app::App, event::Key};

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Char('q') => {
            app.should_quit = true;
        }
        _ => {}
    }
}
