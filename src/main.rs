use anyhow::Result;
use clap::Parser;
use cli::Cli;
use term::Term;

mod app;
mod cli;
mod db;
mod event;
mod handlers;
mod term;
mod ui;

fn main() -> Result<()> {
    let args = Cli::parse();
    Term::run(args.tick_rate_milliseconds, args.custom_db_path)?;
    Ok(())
}
