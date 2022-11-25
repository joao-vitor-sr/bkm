use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use term::Term;

mod app;
mod cli;
mod term;
mod ui;

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_timeout = Duration::from_millis(args.input_timeout);
    Term::run(input_timeout)?;
    Ok(())
}
