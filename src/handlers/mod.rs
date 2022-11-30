mod home;
mod input;

use crate::{
    app::{ActiveBlock, App},
    event::Key,
};
use anyhow::Result;

pub fn handle_app(key: Key, app: &mut App) -> Result<()> {
    match key {
        Key::Char('q') => {
            app.should_quit = true;
        }
        _ => handle_block_input(key, app)?,
    }
    Ok(())
}

fn handle_block_input(key: Key, app: &mut App) -> Result<()> {
    let current_route = app.get_current_route();
    match current_route.block {
        ActiveBlock::Input => {
            input::handler(key, app)?;
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        _ => {}
    }
    Ok(())
}
