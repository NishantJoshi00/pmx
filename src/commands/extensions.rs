use std::process::Command;

use anyhow::{Context, ensure};

use crate::storage::Storage;

pub fn execute_extension(storage: &Storage, args: &[String]) -> crate::Result<()> {
    ensure!(!args.is_empty(), "Extension subcommand cannot be empty");

    let subcommand = &args[0];
    let extension_args = &args[1..];

    // Validate subcommand name to prevent path traversal attacks
    ensure!(
        is_valid_subcommand_name(subcommand),
        "Invalid subcommand name: {}",
        subcommand
    );

    // Check if extension is allowed in configuration
    ensure!(
        storage.is_extension_allowed(subcommand),
        "Extension '{}' is not allowed. Add it to the 'allowed_subcommands' list in config.toml",
        subcommand
    );

    let binary_name = format!("pmx-{subcommand}");

    // Try to execute the extension binary
    let mut command = Command::new(&binary_name);
    command.args(extension_args);

    let status = command
        .status()
        .with_context(|| format!("Failed to execute extension '{binary_name}'"))?;

    // Forward the exit code from the extension
    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            // Process was terminated by signal
            std::process::exit(1);
        }
    }

    Ok(())
}

fn is_valid_subcommand_name(name: &str) -> bool {
    // Only allow alphanumeric characters, hyphens, and underscores
    // This prevents path traversal and other security issues
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{Agents, Config, ExtensionsConfig, McpConfig};
    use tempfile::TempDir;

    fn create_test_storage_with_extensions(allowed_subcommands: Vec<String>) -> (TempDir, Storage) {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_storage");

        // Initialize storage
        std::fs::create_dir_all(&path).unwrap();
        std::fs::create_dir_all(path.join("repo")).unwrap();

        let config = Config {
            agents: Agents {
                disable_claude: false,
                disable_codex: false,
            },
            mcp: McpConfig::default(),
            extensions: ExtensionsConfig {
                allowed_subcommands,
            },
        };

        config.persist(&path).unwrap();
        let storage = Storage::new(path).unwrap();

        (temp_dir, storage)
    }

    #[test]
    fn test_is_valid_subcommand_name() {
        assert!(is_valid_subcommand_name("test"));
        assert!(is_valid_subcommand_name("test-command"));
        assert!(is_valid_subcommand_name("test_command"));
        assert!(is_valid_subcommand_name("test123"));

        assert!(!is_valid_subcommand_name(""));
        assert!(!is_valid_subcommand_name("-test"));
        assert!(!is_valid_subcommand_name("test-"));
        assert!(!is_valid_subcommand_name("test--command"));
        assert!(!is_valid_subcommand_name("test/command"));
        assert!(!is_valid_subcommand_name("test\\command"));
        assert!(!is_valid_subcommand_name("test command"));
        assert!(!is_valid_subcommand_name("test.command"));
    }

    #[test]
    fn test_execute_extension_empty_args() {
        let (_temp_dir, storage) = create_test_storage_with_extensions(vec![]);
        let result = execute_extension(&storage, &[]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Extension subcommand cannot be empty")
        );
    }

    #[test]
    fn test_execute_extension_invalid_name() {
        let (_temp_dir, storage) = create_test_storage_with_extensions(vec![]);
        let result = execute_extension(&storage, &["../malicious".to_string()]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid subcommand name")
        );
    }

    #[test]
    fn test_execute_extension_not_allowed() {
        let (_temp_dir, storage) =
            create_test_storage_with_extensions(vec!["allowed-cmd".to_string()]);
        let result = execute_extension(&storage, &["not-allowed".to_string()]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Extension 'not-allowed' is not allowed")
        );
    }

    #[test]
    fn test_execute_extension_allowed_but_not_found() {
        let (_temp_dir, storage) =
            create_test_storage_with_extensions(vec!["test-cmd".to_string()]);
        let result = execute_extension(&storage, &["test-cmd".to_string()]);
        assert!(result.is_err());
        // Should fail because pmx-test-cmd binary doesn't exist
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to execute extension")
        );
    }
}
