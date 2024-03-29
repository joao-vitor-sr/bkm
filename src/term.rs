use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, path::PathBuf};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::{
    app::{ActiveBlock, App},
    event::{self, Key},
    handlers,
    ui::{input_layout::UiInputLayout, Ui},
};

#[derive(Debug)]
pub struct Term {}

impl Term {
    pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
        let events = event::Events::new(app.tick_rate_milliseconds);

        Ok(loop {
            terminal.draw(|f| match app.get_current_route().block {
                ActiveBlock::Confirm => {
                    Ui::draw_confirm(f, &mut app);
                }
                ActiveBlock::Input => UiInputLayout::draw(f, &mut app),
                _ => Ui::draw_main_layout(f, &mut app),
            })?;

            match events.next()? {
                event::Event::Input(key) => {
                    if key == Key::Ctrl('c') {
                        app.should_quit = true;
                        break;
                    }

                    handlers::handle_app(key, &mut app)?;
                }
                _ => {}
            }

            if app.should_quit {
                return Ok(());
            }
        })
    }

    pub fn run(tick_rate_milliseconds: Option<u64>, custom_db_path: Option<PathBuf>) -> Result<()> {
        // Terminal initialization
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let app = App::new(tick_rate_milliseconds, custom_db_path)?;
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
