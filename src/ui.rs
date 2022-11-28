use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, InputMode};

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

        Ui::render_first_split(f, app, chunks[0]);
        Ui::render_second_split(f, app, chunks[1]);
    }

    pub fn render_first_split<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        f.render_stateful_widget(Ui::draw_books(app), area, &mut app.books_list.state);
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
        let input = Paragraph::new(app.input.as_ref())
            .style(match app.add_book_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Book Name"));

        f.render_widget(input, area);

        match app.add_book_mode {
            InputMode::Normal => {},
            InputMode::Editing => {
                f.set_cursor(area.x + app.input.width() as u16 + 1, area.y + 1)
            }
        }
    }

    pub fn render_msg<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let (msg, style) = match app.add_book_mode {
            InputMode::Normal => Ui::home_msg(),
            InputMode::Editing => Ui::add_book_msg(),
        };

        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_msg =
            Paragraph::new(text).block(Block::default().title("Home").borders(Borders::ALL));
        f.render_widget(help_msg, area);
    }

    pub fn add_book_msg<'a>() -> (Vec<Span<'a>>, Style) {
        let msg = vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to stop editing, "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to record the message"),
        ];
        (msg, Style::default())
    }

    pub fn home_msg<'a>() -> (Vec<Span<'a>>, Style) {
        let home = vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to add a book"),
        ];
        (home, Style::default().add_modifier(Modifier::RAPID_BLINK))
    }

    pub fn draw_books<'a>(app: &App<'a>) -> List<'a> {
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

        items
    }
}
