use anyhow::Result;
use clap::Parser;
use flow::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Note { note } => flow::commands::run_note(note)?,
        Commands::Status => flow::commands::run_status()?,
        Commands::Resume => flow::commands::run_resume()?,
        Commands::History { limit } => flow::commands::run_history(limit)?,
        Commands::Done => flow::commands::run_done()?,
    }

    Ok(())
}
