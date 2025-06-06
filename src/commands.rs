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

pub fn set_claude_profile(storage: &crate::storage::Storage, profile: &str) -> crate::Result<()> {
    let repo_path = storage.path.join("repo");
    let source_file = repo_path.join(format!("{}.md", profile));
    
    if !source_file.exists() {
        anyhow::bail!("Profile '{}' not found at {}", profile, source_file.display());
    }
    
    let claude_dir = std::env::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?
        .join(".claude");
    
    let system_prompt_location = claude_dir.join("CLAUDE.md");
    
    std::fs::create_dir_all(&claude_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create .claude directory: {}", e))?;
    
    std::fs::copy(&source_file, &system_prompt_location)
        .map_err(|e| anyhow::anyhow!("Failed to apply profile '{}': {}", profile, e))?;
    
    println!("Successfully applied profile '{}' to {}", profile, system_prompt_location.display());
    Ok(())
}

pub fn reset_claude_profile(_storage: &crate::storage::Storage) -> crate::Result<()> {
    let system_prompt_location = std::env::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?
        .join(".claude")
        .join("CLAUDE.md");
    
    if system_prompt_location.exists() {
        std::fs::remove_file(&system_prompt_location)
            .map_err(|e| anyhow::anyhow!("Failed to remove {}: {}", system_prompt_location.display(), e))?;
        println!("Successfully reset Claude profile (removed {})", system_prompt_location.display());
    } else {
        println!("No Claude profile found at {} (already reset)", system_prompt_location.display());
    }
    
    Ok(())
}

pub fn completion(shell: &crate::cli::Shell) -> crate::Result<()> {
    match shell {
        crate::cli::Shell::Zsh => {
            const ZSH_COMPLETION: &str = include_str!("../completions/_pmx");
            print!("{}", ZSH_COMPLETION);
        }
    }
    Ok(())
}
