use anyhow::ensure;

pub fn set_codex_profile(storage: &crate::storage::Storage, profile: &str) -> crate::Result<()> {
    ensure!(
        !storage.config.agents.disable_codex,
        "Codex profiles are disabled in the configuration."
    );

    let repo_path = storage.path.join("repo");
    let source_file = repo_path.join(format!("{}.md", profile));

    if !source_file.exists() {
        anyhow::bail!(
            "Profile '{}' not found at {}",
            profile,
            source_file.display()
        );
    }

    let codex_dir = crate::utils::home_dir()?.join(".codex");

    let system_prompt_location = codex_dir.join("AGENTS.md");

    std::fs::create_dir_all(&codex_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create .codex directory: {}", e))?;

    std::fs::copy(&source_file, &system_prompt_location)
        .map_err(|e| anyhow::anyhow!("Failed to apply profile '{}': {}", profile, e))?;

    println!(
        "Successfully applied profile '{}' to {}",
        profile,
        system_prompt_location.display()
    );
    Ok(())
}

pub fn reset_codex_profile(storage: &crate::storage::Storage) -> crate::Result<()> {
    ensure!(
        !storage.config.agents.disable_codex,
        "Codex profiles are disabled in the configuration."
    );

    let system_prompt_location = crate::utils::home_dir()?.join(".codex").join("AGENTS.md");

    if system_prompt_location.exists() {
        std::fs::remove_file(&system_prompt_location).map_err(|e| {
            anyhow::anyhow!(
                "Failed to remove {}: {}",
                system_prompt_location.display(),
                e
            )
        })?;
        println!(
            "Successfully reset Codex profile (removed {})",
            system_prompt_location.display()
        );
    } else {
        println!(
            "No Codex profile found at {} (already reset)",
            system_prompt_location.display()
        );
    }

    Ok(())
}

pub fn append_codex_profile(storage: &crate::storage::Storage, profile: &str) -> crate::Result<()> {
    ensure!(
        !storage.config.agents.disable_codex,
        "Codex profiles are disabled in the configuration."
    );

    let repo_path = storage.path.join("repo");
    let source_file = repo_path.join(format!("{}.md", profile));

    if !source_file.exists() {
        anyhow::bail!(
            "Profile '{}' not found at {}",
            profile,
            source_file.display()
        );
    }

    let codex_dir = crate::utils::home_dir()?.join(".codex");
    let system_prompt_location = codex_dir.join("AGENTS.md");

    std::fs::create_dir_all(&codex_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create .codex directory: {}", e))?;

    let profile_content = std::fs::read_to_string(&source_file)
        .map_err(|e| anyhow::anyhow!("Failed to read profile '{}': {}", profile, e))?;

    if system_prompt_location.exists() {
        let existing_content = std::fs::read_to_string(&system_prompt_location)
            .map_err(|e| anyhow::anyhow!("Failed to read existing Codex profile: {}", e))?;

        let combined_content = format!("{}\n\n{}", existing_content, profile_content);

        std::fs::write(&system_prompt_location, combined_content)
            .map_err(|e| anyhow::anyhow!("Failed to append profile '{}': {}", profile, e))?;

        println!(
            "Successfully appended profile '{}' to {}",
            profile,
            system_prompt_location.display()
        );
    } else {
        std::fs::write(&system_prompt_location, profile_content)
            .map_err(|e| anyhow::anyhow!("Failed to create profile '{}': {}", profile, e))?;

        println!(
            "Successfully created profile '{}' at {} (no existing profile found)",
            profile,
            system_prompt_location.display()
        );
    }

    Ok(())
}
