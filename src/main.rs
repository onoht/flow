use anyhow::Result;
use clap::Parser;
use clap_complete::{generate, Shell};
use flow_ctx::cli::{Cli, Commands};
use std::io;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Note { note } => flow_ctx::commands::run_note(note)?,
        Commands::Status => flow_ctx::commands::run_status()?,
        Commands::Resume => flow_ctx::commands::run_resume()?,
        Commands::History {
            limit,
            search,
            repo,
        } => flow_ctx::commands::run_history(limit, search, repo)?,
        Commands::Done => flow_ctx::commands::run_done()?,
        Commands::Completions { shell } => {
            generate_completions(shell);
        }
    }

    Ok(())
}

fn generate_completions(shell: Shell) {
    let mut cmd = <flow_ctx::cli::Cli as clap::CommandFactory>::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
}
