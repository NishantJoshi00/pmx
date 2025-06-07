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
        // utils
        cli::Command::List => {
            pmx::commands::utils::list(&storage)?;
        }
        cli::Command::CopyProfile(profile) => {
            pmx::commands::utils::copy_profile(&profile.path, &storage)?;
        }
        cli::Command::Completion(completion) => {
            pmx::commands::utils::completion(&completion.shell)?;
        }

        // claude_code
        cli::Command::SetClaudeProfile(profile) => {
            pmx::commands::claude_code::set_claude_profile(&storage, &profile.path)?;
        }
        cli::Command::ResetClaudeProfile => {
            pmx::commands::claude_code::reset_claude_profile(&storage)?;
        }

        // openai_codex
        cli::Command::SetCodexProfile(profile) => {
            pmx::commands::openai_codex::set_codex_profile(&storage, &profile.path)?;
        }
        cli::Command::ResetCodexProfile => {
            pmx::commands::openai_codex::reset_codex_profile(&storage)?;
        }
    }

    Ok(())
}
