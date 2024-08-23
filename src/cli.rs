use std::{path::PathBuf, str::FromStr};

use clap::{builder::OsStr, Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "midpad", version, about = "Command line utility to pad texts.", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub raw: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(short, long, default_value = PadMode::Middle)]
    pub mode: Option<PadMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum, Default)]
pub enum PadMode {
    Left,
    Right,
    #[default]
    Middle,
}

impl FromStr for PadMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "l" | "left" => Ok(PadMode::Left),
            "m" | "middle" => Ok(PadMode::Left),
            "r" | "right" => Ok(PadMode::Left),
            _ => Err(()),
        }
    }
}

impl From<PadMode> for OsStr {
    fn from(value: PadMode) -> Self {
        match value {
            PadMode::Middle => OsStr::from("m"),
            PadMode::Left => OsStr::from("l"),
            PadMode::Right => OsStr::from("r"),
        }
    }
}
