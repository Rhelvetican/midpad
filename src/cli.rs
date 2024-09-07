use std::{path::PathBuf, str::FromStr};

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        OsStr, PossibleValue, Styles,
    },
    Parser, ValueEnum,
};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(
        AnsiColor::Magenta
            .on_default()
            .effects(Effects::UNDERLINE)
            .bold(),
    )
    .placeholder(AnsiColor::Magenta.on_default());

#[derive(Parser)]
#[command(name = "midpad", version, about = "Command line utility to pad texts.", long_about = None)]
#[command(author)]
#[command(styles = STYLES)]
pub struct Cli {
    #[arg(short, long)]
    pub raw: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(short, long, value_enum)]
    pub mode: Option<PadMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

impl ValueEnum for PadMode {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        if !ignore_case {
            match input {
                "l" | "left" => Ok(PadMode::Left),
                "m" | "middle" => Ok(PadMode::Left),
                "r" | "right" => Ok(PadMode::Left),
                _ => Err(String::from("Failed to parse input!")),
            }
        } else {
            let input = input.to_lowercase();

            match input.as_str() {
                "l" | "left" => Ok(PadMode::Left),
                "m" | "middle" => Ok(PadMode::Left),
                "r" | "right" => Ok(PadMode::Left),
                _ => Err(String::from("Failed to parse input!")),
            }
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[PadMode::Middle, PadMode::Left, PadMode::Right]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            PadMode::Middle => Some(
                PossibleValue::new("middle")
                    .help("Middle-pad texts.")
                    .alias("m"),
            ),
            PadMode::Left => Some(
                PossibleValue::new("left")
                    .help("Left-pad texts.")
                    .alias("l"),
            ),
            PadMode::Right => Some(
                PossibleValue::new("right")
                    .help("Right-pad texts.")
                    .alias("r"),
            ),
        }
    }
}
