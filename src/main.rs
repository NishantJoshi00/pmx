use std::path::PathBuf;

use clap::Parser;
use pmx::cli;

fn main() -> anyhow::Result<()> {
    let args = cli::Arg::parse();
    let storage = args
        .config
        .or_else(|| std::env::var("PMX_CONFIG_FILE").ok().map(PathBuf::from))
        .map(pmx::storage::Storage::new)
        .unwrap_or_else(pmx::storage::Storage::auto)?;

    match args.command {
        cli::Command::List => {
            pmx::commands::list(&storage)?;
        }
        cli::Command::SetClaudeProfile(profile) => {
            pmx::commands::set_claude_profile(&storage, &profile.path)?;
        }
        cli::Command::ResetClaudeProfile => {
            pmx::commands::reset_claude_profile(&storage)?;
        }
        cli::Command::Completion(completion) => {
            pmx::commands::completion(&completion.shell)?;
        }
    }

    Ok(())
}
