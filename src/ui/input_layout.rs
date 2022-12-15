use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{app::App, db::books::BookInputs};

#[derive(Debug)]
pub struct UiInputLayout {}

impl UiInputLayout {
    pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(f.size());

        UiInputLayout::render_input_name(f, app, chunks[0]);
        UiInputLayout::render_input_author(f, app, chunks[1]);
        UiInputLayout::render_input_date(f, app, chunks[2]);
    }

    pub fn render_input_date<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let focused = match app.selected_input {
            BookInputs::Date => true,
            _ => false,
        };

        let lines = Text::from(app.book.date.as_str());

        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Date (YYYY-MM-DD)")
                .border_style(Style::default().fg(match focused {
                    true => app.theme.active,
                    _ => app.theme.inactive,
                })),
        );

        f.render_widget(input, area);

        if !focused {
            return;
        }

        f.set_cursor(area.x + app.book.date.width() as u16 + 1, area.y + 1);
    }

    pub fn render_input_author<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let focused = match app.selected_input {
            BookInputs::Author => true,
            _ => false,
        };

        let lines = Text::from(app.book.author.as_str());

        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Author")
                .border_style(Style::default().fg(match focused {
                    true => app.theme.active,
                    _ => app.theme.inactive,
                })),
        );

        f.render_widget(input, area);

        if !focused {
            return;
        }

        f.set_cursor(area.x + app.book.author.width() as u16 + 1, area.y + 1);
    }

    pub fn render_input_name<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let focused = match app.selected_input {
            BookInputs::Name => true,
            _ => false,
        };

        let lines = Text::from(app.book.name.as_str());

        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Name")
                .border_style(Style::default().fg(match focused {
                    true => app.theme.active,
                    false => app.theme.inactive,
                })),
        );

        f.render_widget(input, area);

        if !focused {
            return;
        }

        f.set_cursor(area.x + app.book.name.width() as u16 + 1, area.y + 1);
    }
}
