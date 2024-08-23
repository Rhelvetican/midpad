use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub raw: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
