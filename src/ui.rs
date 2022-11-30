use crate::app::{ActiveBlock, App};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use unicode_width::UnicodeWidthStr;

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

        Ui::render_books(f, app, chunks[0]);
        Ui::render_second_split(f, app, chunks[1]);
    }

    pub fn render_second_split<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

        Ui::render_msg(f, app, vertical_layout[0]);
        Ui::render_input(f, app, vertical_layout[1]);
    }

    pub fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let input_mode = match app.get_current_route().block {
            ActiveBlock::Input => true,
            _ => false,
        };

        let input = Paragraph::new(app.input.as_ref())
            .style(match input_mode {
                false => Style::default(),
                true => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Book Name"));

        f.render_widget(input, area);

        match input_mode {
            false => {}
            true => f.set_cursor(area.x + app.input.width() as u16 + 1, area.y + 1),
        }
    }

    pub fn render_msg<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let (msg, style) = match app.get_current_route().block {
            ActiveBlock::Input => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default(),
            ),
            _ => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to add a book"),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
        };

        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_msg =
            Paragraph::new(text).block(Block::default().title("Home").borders(Borders::ALL));
        f.render_widget(help_msg, area);
    }

    pub fn render_books<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
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
            .block(Block::default().borders(Borders::ALL).title("Books"))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" ");

        f.render_stateful_widget(items, area, &mut app.books_list.state);
    }
}
