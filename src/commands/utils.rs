pub fn list(storage: &crate::storage::Storage) -> crate::Result<()> {
    let profile_list = storage.list_repos()?;

    if profile_list.is_empty() {
        println!("No profiles found.");
        return Ok(());
    }

    profile_list
        .iter()
        .for_each(|profile| println!("{}", profile));
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
            println!("list");
            println!("copy-profile");
            println!("completion");
            
            // Agent-specific commands
            if !storage.config.agents.disable_claude {
                println!("set-claude-profile");
                println!("reset-claude-profile");
            }
            if !storage.config.agents.disable_codex {
                println!("set-codex-profile");
                println!("reset-codex-profile");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{Config, Agents};
    use tempfile::TempDir;
    use std::fs;

    fn create_test_storage(disable_claude: bool, disable_codex: bool) -> (TempDir, crate::storage::Storage) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let repo_dir = temp_dir.path().join("repo");
        
        fs::create_dir(&repo_dir).unwrap();
        
        let config = Config {
            agents: Agents {
                disable_claude,
                disable_codex,
                disable_cline: false,
            },
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
}
