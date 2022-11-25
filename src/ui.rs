use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

#[derive(Debug)]
pub struct Ui {}

impl Ui {
    pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let size = f.size();
        let block = Block::default().title(app.title).borders(Borders::ALL);
        f.render_widget(block, size);
    }
}
