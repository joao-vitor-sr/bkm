use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, path::PathBuf, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::{app::App, ui::Ui};

#[derive(Debug)]
pub struct Term {}

impl Term {
    pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
        loop {
            terminal.draw(|f| Ui::draw(f, &mut app))?;

            if crossterm::event::poll(app.input_timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    }
                }
            }

            if app.should_quit {
                return Ok(());
            }
        }
    }

    pub fn run(input_timeout: Duration, custom_db_path: Option<PathBuf>) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let app = App::new("BKM", input_timeout, custom_db_path)?;
        let res = Term::run_app(&mut terminal, app);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            eprintln!("{:?}", err)
        }

        Ok(())
    }
}
