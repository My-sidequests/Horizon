mod cli;
mod commands;
mod exercises;
mod progress;
mod ui;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Start)        => commands::start(),
        Some(Command::Update)       => commands::update(),
        Some(Command::Done { flag }) => commands::done(flag),
        Some(Command::Hint)         => commands::hint(),
        Some(Command::Skip)         => commands::skip(),
        Some(Command::Reset)        => commands::reset(),
        Some(Command::List)         => commands::list(),
        None                        => commands::show(),
    }
}
