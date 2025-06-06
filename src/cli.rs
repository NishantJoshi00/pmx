use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
pub struct Arg {
    /// Path to the storage directory
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    SetClaudeProfile(ClaudeProfile),
    ResetClaudeProfile,
    List,
    Completion(CompletionArgs),
}

#[derive(Debug, Args)]
pub struct ClaudeProfile {
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
