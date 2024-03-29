mod books;
mod confirm;
mod input;

mod common_key_events;

use crate::{
    app::{ActiveBlock, App},
    event::Key,
};
use anyhow::Result;

pub fn handle_app(key: Key, app: &mut App) -> Result<()> {
    handle_blocks_input(key, app)?;
    Ok(())
}

fn handle_blocks_input(key: Key, app: &mut App) -> Result<()> {
    let current_route = app.get_current_route();
    match current_route.block {
        ActiveBlock::Input => {
            input::handler(key, app)?;
        }
        ActiveBlock::Books => {
            books::handler(key, app);
        }
        ActiveBlock::Confirm => {
            confirm::handler(key, app)?;
        }
    }
    Ok(())
}
