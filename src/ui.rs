use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub mod list;

#[derive(Debug)]
pub struct Ui {}

impl Ui {
    pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let size = f.size();

        let items: Vec<ListItem> = app
            .books_list
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(i.0.clone())];
                ListItem::new(lines).style(Style::default())
            })
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(app.title))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" ");

        f.render_stateful_widget(items, size, &mut app.books_list.state);
    }
}
