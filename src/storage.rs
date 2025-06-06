use std::path::PathBuf;

use anyhow::ensure;

#[derive(Debug, Clone)]
pub struct Storage {
    pub(crate) path: PathBuf,
    pub(crate) config: Config,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) agents: Agents,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Agents {
    pub(crate) disable_claude: bool,
    pub(crate) disable_codex: bool,
    pub(crate) disable_cline: bool,
}

impl Config {
    pub fn persist(&self, path: &PathBuf) -> crate::Result<()> {
        let config_path = path.join("config.toml");
        let config_content = toml::to_string(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize config: {}", e))?;
        std::fs::write(&config_path, config_content)
            .map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;
        Ok(())
    }
    pub fn load(path: &PathBuf) -> crate::Result<Self> {
        let config_path = path.join("config.toml");
        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "Config file does not exist: {}",
                config_path.display()
            ));
        }

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

        Ok(config)
    }
}

impl Storage {
    pub fn new(path: PathBuf) -> crate::Result<Self> {
        Self::validate(&path)?;
        let config = Config::load(&path)?;
        let storage = Self { path, config };
        Ok(storage)
    }

    fn validate(path: &PathBuf) -> crate::Result<()> {
        ensure!(
            path.exists(),
            "Storage path does not exist: {}",
            path.display()
        );

        ensure!(
            path.is_dir(),
            "Storage path is not a directory: {}",
            path.display()
        );

        let repo_path = path.join("repo");
        ensure!(
            repo_path.exists(),
            "Repository path does not exist: {}",
            repo_path.display()
        );

        ensure!(
            repo_path.is_dir(),
            "Repository path is not a directory: {}",
            repo_path.display()
        );

        let config_path = path.join("config.toml");
        ensure!(
            config_path.exists(),
            "Config file does not exist: {}",
            config_path.display()
        );

        ensure!(
            config_path.is_file(),
            "Config path is not a file: {}",
            config_path.display()
        );

        Ok(())
    }

    pub(crate) fn initialize(path: PathBuf) -> crate::Result<Self> {
        ensure!(
            !path.exists(),
            "Storage path already exists: {}",
            path.display()
        );
        std::fs::create_dir_all(&path)
            .map_err(|e| anyhow::anyhow!("Failed to create storage directory: {}", e))?;

        let repo = path.join("repo");

        std::fs::create_dir_all(&repo)
            .map_err(|e| anyhow::anyhow!("Failed to create repo directory: {}", e))?;

        let config = Config {
            agents: Agents {
                disable_claude: false,
                disable_codex: false,
                disable_cline: false,
            },
        };

        config.persist(&path)?;
        Self::validate(&path)?;
        let storage = Self { path, config };

        Ok(storage)
    }

    pub fn list_repos(&self) -> crate::Result<Vec<String>> {
        let repo_path = self.path.join("repo");
        let list = recursive_list(&repo_path)
            .map_err(|e| anyhow::anyhow!("Failed to list repositories: {}", e))?;
        let list = list
            .into_iter()
            .filter(|path| path.is_file())
            .filter(|path| path.extension().map(|e| e == "md").unwrap_or(false))
            .map(|path| {
                path.strip_prefix(&repo_path)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| path.to_string_lossy().to_string())
            })
            .map(|s| s.trim_end_matches(".md").to_string())
            .collect();
        Ok(list)
    }

    pub fn auto() -> crate::Result<Self> {
        let xdg_data_home = std::env::var("XDG_CONFIG_HOME").ok();
        let other_path = crate::utils::home_dir()
            .map(|p| p.join(".config/pmx"))
            .expect("Failed to get home directory");

        let path = xdg_data_home
            .map(PathBuf::from)
            .unwrap_or_else(|| other_path.clone());

        Self::new(path).or_else(|e| {
            eprintln!("Failed to load storage from {:?}: {}", other_path, e);
            Self::initialize(other_path)
        })
    }
}

fn recursive_list(path: &PathBuf) -> crate::Result<Vec<PathBuf>> {
    match path {
        path if path.is_dir() => {
            let list = std::fs::read_dir(path)
                .map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?;

            Ok(list
                .filter_map(|entry| Some(entry.ok()?.path()))
                .filter(|p| p.is_file() || p.is_dir())
                .map(|path| recursive_list(&path))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect())
        }

        path if path.is_file() => Ok(vec![path.clone()]),

        _ => Err(anyhow::anyhow!(
            "Path is neither a file nor a directory: {}",
            path.display()
        )),
    }
}
