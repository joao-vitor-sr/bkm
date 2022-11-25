use anyhow::Result;
use term::Term;

mod term;
mod app;
mod ui;

fn main() -> Result<()> {
    Term::run()?;
    Ok(())
}
