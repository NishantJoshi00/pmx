pub fn home_dir() -> anyhow::Result<std::path::PathBuf> {
    #[cfg(windows)]
    {
        anyhow::bail!(
            "Home directory retrieval is not supported on Windows. Please set the environment variable manually."
        );
    }

    #[allow(deprecated)]
    std::env::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))
}
