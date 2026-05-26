use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "horizon")]
#[command(about = "Learn security through hands-on exploitation")]
#[command(version = "0.1.0")]
#[command(arg_required_else_help = false)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Compile all exercises and begin — run this first
    Start,

    /// Pull the latest version from GitHub and recompile
    Update,

    /// Validate the current exercise with its flag
    Done {
        /// The flag you found (format: HRZ{...})
        flag: String,
    },

    /// Reveal the next hint for the current exercise
    Hint,

    /// Skip the current exercise
    Skip,

    /// Reset all progress (asks for confirmation)
    Reset,

    /// List all exercises with their completion status
    List,
}
