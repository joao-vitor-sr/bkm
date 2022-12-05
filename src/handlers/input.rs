use anyhow::Result;

use crate::{app::App, db::books::Book, event::Key};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

fn compute_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}

fn clear_input(app: &mut App) {
    // clear input
    app.input = vec![];
    app.input_idx = 0;
    app.input_cursor_position = 0;
}

fn process_input(app: &mut App, input: String) -> Result<()> {
    // Don't do anything if there is no input
    if input.is_empty() {
        return Ok(());
    }

    match &app.selected_book_id {
        Some(v) => {
            Book::update_book(&app.db.db_file_path, &v, &input)?;
            let book_index = match app.selected_book_index {
                None => 0,
                Some(i) => i,
            };

            app.books[book_index].name = input.clone();
            app.selected_book_id = None;

            clear_input(app);
            app.reset_navigation_stack();
            return Ok(());
        }
        None => {}
    };
    let (id, name) = Book::insert_book(&app.db.db_file_path, input)?;
    app.books.push(Book { name, id });

    // clear input
    clear_input(app);

    Ok(())
}

pub fn handler(key: Key, app: &mut App) -> Result<()> {
    match key {
        Key::Ctrl('k') => {
            app.input.drain(app.input_idx..app.input.len());
        }
        Key::Ctrl('u') => {
            app.input.drain(..app.input_idx);
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('l') => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('w') => {
            if app.input_cursor_position == 0 {
                return Ok(());
            }
            let word_end = match app.input[..app.input_idx].iter().rposition(|&x| x != ' ') {
                Some(index) => index + 1,
                None => 0,
            };

            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };

            let deleted: String = app.input[word_start..app.input_idx].iter().collect();
            let deleted_len: u16 = UnicodeWidthStr::width(deleted.as_str()).try_into().unwrap();
            app.input.drain(word_start..app.input_idx);
            app.input_idx = word_start;
            app.input_cursor_position -= deleted_len;
        }

        Key::End | Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }

        Key::Home | Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }

        Key::Left | Key::Ctrl('b') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }

        Key::Right | Key::Ctrl('f') => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                app.input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Esc => {
            app.reset_navigation_stack();
        }

        Key::Enter => {
            let input_str: String = app.input.iter().collect();

            process_input(app, input_str)?;
        }
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            app.input_cursor_position += compute_character_width(c);
        }

        Key::Backspace | Key::Ctrl('h') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Delete | Key::Ctrl('d') => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }

        _ => {}
    }
    Ok(())
}
