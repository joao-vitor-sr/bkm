use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub mod list;

#[derive(Debug)]
pub struct Ui {}

impl Ui {
    pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        f.render_stateful_widget(Ui::render_books(app), chunks[0], &mut app.books_list.state);
        f.render_widget(Ui::render_home(app), chunks[1]);
    }

    pub fn render_home<'a>(app: &'a App) -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                app.title,
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Press 'a' to add a new book")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(app.title),
        );
        home
    }

    pub fn render_books<'a>(app: &App<'a>) -> List<'a> {
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

        items
    }
}
