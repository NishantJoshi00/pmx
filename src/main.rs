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
        cli::Command::Completion(completion) => {
            pmx::commands::utils::completion(&completion.shell)?;
        }

        // profile management
        cli::Command::Profile(profile_cmd) => match profile_cmd {
            cli::ProfileCommand::List => {
                pmx::commands::utils::list(&storage)?;
            }
            cli::ProfileCommand::Edit(args) => {
                pmx::commands::profile::edit(&storage, &args.name)?;
            }
            cli::ProfileCommand::Delete(args) => {
                pmx::commands::profile::delete(&storage, &args.name)?;
            }
            cli::ProfileCommand::Create(args) => {
                pmx::commands::profile::create(&storage, &args.name)?;
            }
            cli::ProfileCommand::Show(args) => {
                pmx::commands::profile::show(&storage, &args.name)?;
            }
            cli::ProfileCommand::Copy(args) => {
                pmx::commands::profile::copy(&storage, &args.name)?;
            }
        },

        // claude_code
        cli::Command::SetClaudeProfile(profile) => {
            pmx::commands::claude_code::set_claude_profile(&storage, &profile.path)?;
        }
        cli::Command::ResetClaudeProfile => {
            pmx::commands::claude_code::reset_claude_profile(&storage)?;
        }
        cli::Command::AppendClaudeProfile(profile) => {
            pmx::commands::claude_code::append_claude_profile(&storage, &profile.path)?;
        }

        // openai_codex
        cli::Command::SetCodexProfile(profile) => {
            pmx::commands::openai_codex::set_codex_profile(&storage, &profile.path)?;
        }
        cli::Command::ResetCodexProfile => {
            pmx::commands::openai_codex::reset_codex_profile(&storage)?;
        }
        cli::Command::AppendCodexProfile(profile) => {
            pmx::commands::openai_codex::append_codex_profile(&storage, &profile.path)?;
        }

        // internal completion
        cli::Command::InternalCompletion(completion_cmd) => {
            pmx::commands::utils::internal_completion(&storage, &completion_cmd)?;
        }

        // MCP server
        cli::Command::Mcp(_args) => {
            pmx::commands::mcp::run_mcp_server(storage)?;
        }
    }

    Ok(())
}
