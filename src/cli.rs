use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(name = "pmx")]
#[command(about = "A prompt management suite")]
#[command(version)]
pub struct Arg {
    /// Path to the storage directory
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Set Claude profile from a stored configuration
    SetClaudeProfile(ClaudeProfile),
    /// Reset the current Claude profile
    ResetClaudeProfile,
    /// Set Codex profile from a stored configuration
    SetCodexProfile(CodexProfile),
    /// Reset the current Codex profile
    ResetCodexProfile,
    /// List all available profiles
    List,
    /// Copy profile contents to clipboard
    CopyProfile(CopyProfile),
    /// Generate shell completions
    Completion(CompletionArgs),
    /// Internal completion commands (hidden)
    #[command(subcommand, hide = true)]
    InternalCompletion(InternalCompletionCommand),
}

#[derive(Debug, Args)]
pub struct ClaudeProfile {
    /// Path to the profile to apply
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CodexProfile {
    /// Path to the profile to apply
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CopyProfile {
    /// Path to the profile to copy
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CompletionArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shell {
    Zsh,
}

#[derive(Debug, Subcommand)]
pub enum InternalCompletionCommand {
    /// List available Claude profiles (internal)
    ClaudeProfiles,
    /// List available Codex profiles (internal)
    CodexProfiles,
    /// List enabled agent commands (internal)
    EnabledCommands,
}
