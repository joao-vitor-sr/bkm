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
    Term::run(args.tick_rate_milliseconds, args.custom_db_path)?;
    Ok(())
}
