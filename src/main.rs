mod cli;
mod consts;

use std::io::stdout;

use clap::Parser;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

fn main() -> anyhow::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::Green),
        Print("Hello World!"),
        ResetColor,
    )?;

    let cli = cli::Cli::parse();
    dbg!(&cli);

    Ok(())
}
