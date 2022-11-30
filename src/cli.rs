use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub tick_rate_milliseconds: Option<u64>,

    #[arg(short, long)]
    pub custom_db_path: Option<PathBuf>,
}
