use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Result;
use clap::Parser;
use cli::{Cli, PadMode};
use midpad::{leftpad, midpad, rightpad};

mod cli;
mod test;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(raw_content) = cli.raw.as_deref() {
        let content = raw_content.lines().map(|s| s.to_string()).collect();

        let new_content = if let Some(mode) = cli.mode {
            match mode {
                PadMode::Middle => midpad(content),
                PadMode::Left => leftpad(content),
                PadMode::Right => rightpad(content),
            }
        } else {
            midpad(content)
        };

        if let Some(path) = cli.output.as_deref() {
            let mut f = File::create(path)?;
            f.write_all(new_content.join("\n").as_bytes())?;
        } else {
            for line in new_content {
                println!("{}", line)
            }
        }
    } else if let Some(input) = cli.file.as_deref() {
        let mut f = File::open(input)?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let content = content.lines().map(|s| s.to_string()).collect();
        let new_content = if let Some(mode) = cli.mode {
            match mode {
                PadMode::Middle => midpad(content),
                PadMode::Left => leftpad(content),
                PadMode::Right => rightpad(content),
            }
        } else {
            midpad(content)
        };

        if let Some(path) = cli.output.as_deref() {
            let mut f = File::create(path)?;
            f.write_all(new_content.join("\n").as_bytes())?;
        } else {
            for line in new_content {
                println!("{}", line)
            }
        }
    }

    Ok(())
}
