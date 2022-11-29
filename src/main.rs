use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use term::Term;

mod app;
mod cli;
mod term;
mod ui;
mod db;
mod handlers;
mod event;

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_timeout = Duration::from_millis(args.input_timeout);
    Term::run(input_timeout, args.custom_db_path)?;
    Ok(())
}
