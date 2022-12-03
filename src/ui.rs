use crate::{
    app::{ActiveBlock, App},
    event::Key,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

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
        let focused = match app.get_current_route().block {
            ActiveBlock::Input => true,
            _ => false,
        };

        let fg_color = match focused {
            true => Color::Yellow,
            false => Color::White,
        };

        let input_string: String = app.input.iter().collect();
        let lines = Text::from((&input_string).as_str());

        let input = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("Book", Style::default().fg(fg_color)))
                .border_style(Style::default().fg(fg_color)),
        );
        f.render_widget(input, area);

        match focused {
            true => f.set_cursor(area.x + app.input.len() as u16 + 1, area.y + 1),
            _ => {}
        }
    }

    pub fn render_msg<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let focused = match app.get_current_route().block {
            ActiveBlock::Home => true,
            _ => false,
        };

        let fg_color = match focused {
            true => Color::Yellow,
            false => Color::White,
        };

        let book_id = match app.selected_book_index {
            Some(v) => v,
            None => 0,
        };

        let (msg, style) = match app.get_current_route().block {
            ActiveBlock::Input => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default().fg(fg_color),
            ),
            ActiveBlock::Books => (
                vec![
                    Span::styled("name: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}, ", app.books[book_id].name)),
                ],
                Style::default()
                    .add_modifier(Modifier::RAPID_BLINK)
                    .fg(fg_color),
            ),
            _ => (
                vec![
                    Span::raw("Press "),
                    Span::styled(
                        format!("{}", Key::Ctrl('c')),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to exit, "),
                    Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to add a book, "),
                    Span::styled("b", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to select a book"),
                ],
                Style::default()
                    .add_modifier(Modifier::RAPID_BLINK)
                    .fg(fg_color),
            ),
        };

        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_msg = Paragraph::new(text).block(
            Block::default()
                .style(Style::default().fg(fg_color))
                .title("Home")
                .borders(Borders::ALL),
        );
        f.render_widget(help_msg, area);
    }

    pub fn render_books<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let no_book_msg = Span::raw("No books found");

        let focused = match app.get_current_route().block {
            ActiveBlock::Books => true,
            _ => false,
        };

        let fg_color = match focused {
            true => Color::Yellow,
            _ => Color::White,
        };

        let items = if app.books.len() == 0 {
            vec![ListItem::new(no_book_msg)]
        } else {
            app.books
                .iter()
                .map(|book| ListItem::new(Span::raw(&book.name)))
                .collect()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title(Span::styled("Books", Style::default().fg(fg_color)))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(fg_color)),
            )
            .style(Style::default().fg(fg_color))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let mut state = ListState::default();
        state.select(app.selected_book_index);
        f.render_stateful_widget(list, area, &mut state);
    }
}
