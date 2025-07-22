pub fn list(storage: &crate::storage::Storage) -> crate::Result<()> {
    use is_terminal::IsTerminal;
    use std::collections::BTreeMap;
    use std::io;

    let profile_list = storage.list_repos()?;

    if profile_list.is_empty() {
        println!("No profiles found.");
        return Ok(());
    }

    // If output is piped, use the simple format
    if !io::stdout().is_terminal() {
        profile_list
            .iter()
            .for_each(|profile| println!("{}", profile));
        return Ok(());
    }

    // For terminal output, create a tree-like structure
    let mut tree: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for profile in &profile_list {
        if let Some(slash_pos) = profile.find('/') {
            let (dir, file) = profile.split_at(slash_pos);
            let file = &file[1..]; // Remove the leading '/'
            tree.entry(dir.to_string())
                .or_default()
                .push(file.to_string());
        } else {
            tree.entry(String::new()).or_default().push(profile.clone());
        }
    }

    // Print the tree
    let dirs: Vec<_> = tree.keys().collect();
    for (i, dir) in dirs.iter().enumerate() {
        let is_last_dir = i == dirs.len() - 1;

        if dir.is_empty() {
            // Root level files
            if let Some(files) = tree.get(*dir) {
                for (j, file) in files.iter().enumerate() {
                    let is_last_file = j == files.len() - 1 && is_last_dir;
                    let prefix = if is_last_file {
                        "└── "
                    } else {
                        "├── "
                    };
                    println!("{}{}", prefix, file);
                }
            }
        } else {
            // Directory
            let dir_prefix = if is_last_dir {
                "└── "
            } else {
                "├── "
            };
            println!("{}{}/", dir_prefix, dir);

            if let Some(files) = tree.get(*dir) {
                for (j, file) in files.iter().enumerate() {
                    let is_last_file = j == files.len() - 1;
                    let file_prefix = if is_last_dir {
                        if is_last_file {
                            "    └── "
                        } else {
                            "    ├── "
                        }
                    } else {
                        if is_last_file {
                            "│   └── "
                        } else {
                            "│   ├── "
                        }
                    };
                    println!("{}{}", file_prefix, file);
                }
            }
        }
    }

    Ok(())
}

pub fn copy_profile(path: &str, storage: &crate::storage::Storage) -> crate::Result<()> {
    use arboard::Clipboard;
    use std::fs;

    let profile_path = storage.get_repo_path(path)?;
    let content = fs::read_to_string(&profile_path)?;

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;

    println!("Profile content copied to clipboard: {}", path);
    Ok(())
}

pub fn completion(shell: &crate::cli::Shell) -> crate::Result<()> {
    match shell {
        crate::cli::Shell::Zsh => {
            const ZSH_COMPLETION: &str = include_str!("../../completions/_pmx");
            print!("{}", ZSH_COMPLETION);
        }
    }
    Ok(())
}

pub fn internal_completion(
    storage: &crate::storage::Storage,
    completion_cmd: &crate::cli::InternalCompletionCommand,
) -> crate::Result<()> {
    match completion_cmd {
        crate::cli::InternalCompletionCommand::ClaudeProfiles => {
            if !storage.config.agents.disable_claude {
                let profile_list = storage.list_repos()?;
                profile_list
                    .iter()
                    .for_each(|profile| println!("{}", profile));
            }
        }
        crate::cli::InternalCompletionCommand::CodexProfiles => {
            if !storage.config.agents.disable_codex {
                let profile_list = storage.list_repos()?;
                profile_list
                    .iter()
                    .for_each(|profile| println!("{}", profile));
            }
        }
        crate::cli::InternalCompletionCommand::EnabledCommands => {
            // Always available commands
            println!("profile");
            println!("completion");

            // Agent-specific commands
            if !storage.config.agents.disable_claude {
                println!("set-claude-profile");
                println!("reset-claude-profile");
                println!("append-claude-profile");
            }
            if !storage.config.agents.disable_codex {
                println!("set-codex-profile");
                println!("reset-codex-profile");
                println!("append-codex-profile");
            }

            // MCP command (only if prompts or tools are enabled)
            if storage.is_mcp_enabled() {
                println!("mcp");
            }
        }
        crate::cli::InternalCompletionCommand::ProfileNames => {
            let profile_list = storage.list_repos()?;
            profile_list
                .iter()
                .for_each(|profile| println!("{}", profile));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{Agents, Config};
    use std::fs;
    use tempfile::TempDir;

    fn create_test_storage(
        disable_claude: bool,
        disable_codex: bool,
    ) -> (TempDir, crate::storage::Storage) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let repo_dir = temp_dir.path().join("repo");

        fs::create_dir(&repo_dir).unwrap();

        let config = Config {
            agents: Agents {
                disable_claude,
                disable_codex,
            },
            mcp: crate::storage::McpConfig::default(),
        };

        let config_content = toml::to_string(&config).unwrap();
        fs::write(&config_path, config_content).unwrap();

        // Create test profile
        let test_profile = repo_dir.join("test_profile.md");
        fs::write(&test_profile, "# Test Profile\nThis is a test profile.").unwrap();

        let storage = crate::storage::Storage::new(temp_dir.path().to_path_buf()).unwrap();
        (temp_dir, storage)
    }

    #[test]
    fn test_internal_completion_claude_profiles_enabled() {
        let (_temp_dir, storage) = create_test_storage(false, false);

        let cmd = crate::cli::InternalCompletionCommand::ClaudeProfiles;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_claude_profiles_disabled() {
        let (_temp_dir, storage) = create_test_storage(true, false);

        let cmd = crate::cli::InternalCompletionCommand::ClaudeProfiles;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_codex_profiles_enabled() {
        let (_temp_dir, storage) = create_test_storage(false, false);

        let cmd = crate::cli::InternalCompletionCommand::CodexProfiles;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_codex_profiles_disabled() {
        let (_temp_dir, storage) = create_test_storage(false, true);

        let cmd = crate::cli::InternalCompletionCommand::CodexProfiles;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_enabled_commands_all_enabled() {
        let (_temp_dir, storage) = create_test_storage(false, false);

        let cmd = crate::cli::InternalCompletionCommand::EnabledCommands;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_enabled_commands_claude_disabled() {
        let (_temp_dir, storage) = create_test_storage(true, false);

        let cmd = crate::cli::InternalCompletionCommand::EnabledCommands;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_enabled_commands_codex_disabled() {
        let (_temp_dir, storage) = create_test_storage(false, true);

        let cmd = crate::cli::InternalCompletionCommand::EnabledCommands;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_enabled_commands_all_disabled() {
        let (_temp_dir, storage) = create_test_storage(true, true);

        let cmd = crate::cli::InternalCompletionCommand::EnabledCommands;
        let result = internal_completion(&storage, &cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_internal_completion_enabled_commands_with_mcp() {
        use std::fs;
        use tempfile::TempDir;

        // Create storage with MCP disabled
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let repo_dir = temp_dir.path().join("repo");
        fs::create_dir(&repo_dir).unwrap();

        let config = crate::storage::Config {
            agents: crate::storage::Agents {
                disable_claude: true,
                disable_codex: true,
            },
            mcp: crate::storage::McpConfig {
                disable_prompts: crate::storage::DisableOption::Bool(true),
                disable_tools: crate::storage::DisableOption::Bool(true),
            },
        };

        let config_content = toml::to_string(&config).unwrap();
        fs::write(&config_path, config_content).unwrap();

        let storage = crate::storage::Storage::new(temp_dir.path().to_path_buf()).unwrap();

        // Since we can't easily capture stdout in unit tests, we'll test the logic directly
        assert!(!storage.is_mcp_enabled());
    }
}
