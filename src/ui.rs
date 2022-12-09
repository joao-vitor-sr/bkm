use crate::{
    app::{ActiveBlock, App},
    event::Key,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

#[derive(Debug)]
pub struct Ui {}

impl Ui {
    pub fn draw_confirm<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let book_id = match app.selected_book_index {
            Some(i) => i,
            None => 0,
        };

        let book = &app.books[book_id];
        let bounds = f.size();
        // maybe do this better
        let width = std::cmp::min(bounds.width - 2, 45);
        let height = 8;
        let left = (bounds.width - width) / 2;
        let top = bounds.height / 4;

        let rect = Rect::new(left, top, width, height);

        f.render_widget(Clear, rect);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(app.theme.active));

        f.render_widget(block, rect);

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
            .split(rect);

        // suggestion: possibly put this as part of
        // app.dialog, but would have to introduce lifetime
        let text = vec![
            Spans::from(Span::raw("Are you sure you want to delete the book: ")),
            Spans::from(Span::styled(
                book.name.as_str(),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Spans::from(Span::raw("?")),
        ];

        let text = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        f.render_widget(text, vchunks[0]);

        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(3)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(vchunks[1]);

        let ok_text = Span::raw("Ok");
        let ok = Paragraph::new(ok_text)
            .style(Style::default().fg(if app.confirm {
                app.theme.active
            } else {
                app.theme.inactive
            }))
            .alignment(Alignment::Center);

        f.render_widget(ok, hchunks[0]);

        let cancel_text = Span::raw("Cancel");
        let cancel = Paragraph::new(cancel_text)
            .style(Style::default().fg(if app.confirm {
                app.theme.inactive
            } else {
                app.theme.active
            }))
            .alignment(Alignment::Center);

        f.render_widget(cancel, hchunks[1]);
    }

    pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        Ui::render_books(f, app, chunks[0]);
        Ui::redner_main_block(f, app, chunks[1]);
    }

    pub fn redner_main_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

        match app.get_current_route().block {
            ActiveBlock::Input => Ui::render_msg_input(f, app, vertical_layout[0]),
            _ => {
                if app.selected_book_index == None {
                    Ui::render_msg_welcome(f, app, vertical_layout[0]);
                } else {
                    Ui::render_msg_book(f, app, vertical_layout[0]);
                }
            }
        };

        Ui::render_input(f, app, vertical_layout[1]);
    }

    pub fn render_msg_book<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let book_id = match app.selected_book_index {
            Some(v) => v,
            None => 0,
        };
        let (msg, style) = (
            vec![
                Span::styled("name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", app.books[book_id].name)),
            ],
            Style::default()
                .add_modifier(Modifier::RAPID_BLINK)
                .fg(app.theme.text),
        );
        Ui::render_msg(f, app, area, msg, style);
    }

    pub fn render_msg_welcome<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let (msg, style) = (
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
                .fg(app.theme.text),
        );
        Ui::render_msg(f, app, area, msg, style);
    }
    pub fn render_msg_input<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let (msg, style) = (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default().fg(app.theme.text),
        );

        Ui::render_msg(f, app, area, msg, style);
    }

    pub fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
        let focused = match app.get_current_route().block {
            ActiveBlock::Input => true,
            _ => false,
        };

        let fg_color = match focused {
            true => app.theme.active,
            false => app.theme.inactive,
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

    pub fn render_msg<B: Backend>(
        f: &mut Frame<B>,
        app: &mut App,
        area: Rect,
        msg: Vec<Span>,
        style: Style,
    ) {
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_msg = Paragraph::new(text).block(
            Block::default()
                .style(Style::default().fg(app.theme.text))
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
            true => app.theme.active,
            _ => app.theme.inactive,
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
                    .fg(app.theme.text)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let mut state = ListState::default();
        state.select(app.selected_book_index);
        f.render_stateful_widget(list, area, &mut state);
    }
}
