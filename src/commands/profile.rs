use anyhow::{Context, anyhow};
use dialoguer::Confirm;
use std::env;
use std::fs;
use std::process::Command;

pub fn edit(storage: &crate::storage::Storage, name: &str) -> crate::Result<()> {
    // Check if profile exists
    let profile_path = storage.get_repo_path(name)?;

    // Get editor from environment or use default
    let editor = get_editor()?;

    // Open profile in editor
    let status = Command::new(&editor)
        .arg(&profile_path)
        .status()
        .with_context(|| format!("Failed to execute editor: {}", editor))?;

    if !status.success() {
        return Err(anyhow!("Editor exited with non-zero status"));
    }

    println!("Profile '{}' edited successfully", name);
    Ok(())
}

pub fn delete(storage: &crate::storage::Storage, name: &str) -> crate::Result<()> {
    // Check if profile exists
    let profile_path = storage.get_repo_path(name)?;

    // Show profile content before deletion
    let content = fs::read_to_string(&profile_path)
        .with_context(|| format!("Failed to read profile: {}", name))?;

    println!("Profile '{}' contents:", name);
    println!("{}", content);
    println!();

    // Ask for confirmation
    let confirmed = Confirm::new()
        .with_prompt(format!("Delete profile '{}'?", name))
        .default(false)
        .interact()
        .with_context(|| "Failed to get confirmation")?;

    if !confirmed {
        println!("Deletion cancelled");
        return Ok(());
    }

    // Delete the profile
    storage.delete_profile(name)?;
    println!("Profile '{}' deleted successfully", name);
    Ok(())
}

pub fn create(storage: &crate::storage::Storage, name: &str) -> crate::Result<()> {
    // Check if profile already exists
    if storage.profile_exists(name) {
        return Err(anyhow!(
            "Profile '{}' already exists. Use 'edit' to modify it.",
            name
        ));
    }

    // Validate profile name
    validate_profile_name(name)?;

    // Create temporary file for editing
    let temp_file =
        tempfile::NamedTempFile::new().with_context(|| "Failed to create temporary file")?;

    // Write initial template content
    let template = format!("# {}\n\n<!-- Add your profile content here -->\n", name);
    fs::write(temp_file.path(), template)
        .with_context(|| "Failed to write template to temporary file")?;

    // Get editor from environment or use default
    let editor = get_editor()?;

    // Open temporary file in editor
    let status = Command::new(&editor)
        .arg(temp_file.path())
        .status()
        .with_context(|| format!("Failed to execute editor: {}", editor))?;

    if !status.success() {
        return Err(anyhow!("Editor exited with non-zero status"));
    }

    // Read the content back from temporary file
    let content = fs::read_to_string(temp_file.path())
        .with_context(|| "Failed to read content from temporary file")?;

    // Check if the content is effectively empty (only whitespace, comments, or original template)
    let trimmed_content = content.trim();
    let template_header = format!("# {}", name);
    let is_empty = trimmed_content.is_empty()
        || trimmed_content == template_header
        || trimmed_content
            == format!(
                "{}\n\n<!-- Add your profile content here -->",
                template_header
            )
            .trim()
        || trimmed_content.lines().all(|line| {
            let line = line.trim();
            line.is_empty() || line.starts_with('#') || line.starts_with("<!--")
        });

    if is_empty {
        println!("Profile creation cancelled - no content added");
        return Ok(());
    }

    // Create the profile
    storage.create_profile(name, &content)?;
    println!("Profile '{}' created successfully", name);
    Ok(())
}

pub fn show(storage: &crate::storage::Storage, name: &str) -> crate::Result<()> {
    let content = storage.get_profile_content(name)?;
    println!("{}", content);
    Ok(())
}

pub fn copy(storage: &crate::storage::Storage, name: &str) -> crate::Result<()> {
    // Reuse the existing copy_profile functionality
    crate::commands::utils::copy_profile(name, storage)
}

fn get_editor() -> crate::Result<String> {
    // Try $EDITOR first
    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }

    // Try $VISUAL as fallback
    if let Ok(editor) = env::var("VISUAL") {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }

    // Platform-specific defaults
    #[cfg(unix)]
    {
        // Try common editors on Unix systems
        for editor in &["vi", "nano", "emacs"] {
            if Command::new("which")
                .arg(editor)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                return Ok(editor.to_string());
            }
        }
    }

    #[cfg(windows)]
    {
        return Ok("notepad".to_string());
    }

    Err(anyhow!(
        "No editor found. Please set the EDITOR environment variable."
    ))
}

fn validate_profile_name(name: &str) -> crate::Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Profile name cannot be empty"));
    }

    if name.len() > 255 {
        return Err(anyhow!("Profile name too long (max 255 characters)"));
    }

    // Check for path traversal attempts
    if name.contains("..") || name.contains('\\') {
        return Err(anyhow!("Profile name cannot contain '..' or backslashes"));
    }

    // Ensure no empty path components when using forward slashes
    if name.contains('/') {
        for component in name.split('/') {
            if component.is_empty() {
                return Err(anyhow!("Profile name cannot have empty path components"));
            }
            if component == "." || component == ".." {
                return Err(anyhow!(
                    "Profile name cannot contain '.' or '..' path components"
                ));
            }
        }
    }

    // Check for invalid characters
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    if name
        .chars()
        .any(|c| invalid_chars.contains(&c) || c.is_control())
    {
        return Err(anyhow!("Profile name contains invalid characters"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{Agents, Config};
    use std::fs;
    use tempfile::TempDir;

    fn create_test_storage() -> (TempDir, crate::storage::Storage) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let repo_dir = temp_dir.path().join("repo");

        fs::create_dir(&repo_dir).unwrap();

        let config = Config {
            agents: Agents {
                disable_claude: false,
                disable_codex: false,
                disable_cline: false,
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
    fn test_validate_profile_name_valid() {
        assert!(validate_profile_name("valid_name").is_ok());
        assert!(validate_profile_name("valid-name").is_ok());
        assert!(validate_profile_name("valid123").is_ok());
        assert!(validate_profile_name("design/plan").is_ok());
        assert!(validate_profile_name("category/subcategory/name").is_ok());
    }

    #[test]
    fn test_validate_profile_name_invalid() {
        assert!(validate_profile_name("").is_err());
        assert!(validate_profile_name("../invalid").is_err());
        assert!(validate_profile_name("invalid\\name").is_err());
        assert!(validate_profile_name("invalid<name").is_err());
        assert!(validate_profile_name(&"x".repeat(256)).is_err());
        assert!(validate_profile_name("invalid/").is_err()); // empty component
        assert!(validate_profile_name("/invalid").is_err()); // empty component
        assert!(validate_profile_name("invalid//name").is_err()); // empty component
        assert!(validate_profile_name("invalid/.").is_err()); // dot component
        assert!(validate_profile_name("invalid/..").is_err()); // dotdot component
    }

    #[test]
    fn test_show_existing_profile() {
        let (_temp_dir, storage) = create_test_storage();
        let result = show(&storage, "test_profile");
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_nonexistent_profile() {
        let (_temp_dir, storage) = create_test_storage();
        let result = show(&storage, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_copy_existing_profile() {
        let (_temp_dir, storage) = create_test_storage();
        let result = copy(&storage, "test_profile");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_editor_with_env() {
        unsafe {
            env::set_var("EDITOR", "test-editor");
            let result = get_editor();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "test-editor");
            env::remove_var("EDITOR");
        }
    }
}
