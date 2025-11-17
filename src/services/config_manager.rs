use crate::models::UserConfig;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn default_path() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "macos") {
            dirs::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                .join("perch")
        } else if cfg!(target_os = "linux") {
            dirs::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                .join("perch")
        } else {
            return Err(anyhow::anyhow!("Unsupported platform"));
        };

        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("perch.toml"))
    }

    pub fn load(&self) -> Result<UserConfig> {
        if !self.config_path.exists() {
            return Ok(UserConfig::default());
        }

        let contents = fs::read_to_string(&self.config_path)?;
        let config: UserConfig =
            toml::from_str(&contents).unwrap_or_else(|_| UserConfig::default());

        Ok(config)
    }

    pub fn save(&self, config: &UserConfig) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(config)?;
        fs::write(&self.config_path, contents)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_nonexistent_returns_default() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.toml");

        let manager = ConfigManager::new(path);
        let config = manager.load().unwrap();

        assert_eq!(config, UserConfig::default());
    }

    #[test]
    fn test_save_creates_dir() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested").join("config.toml");

        let manager = ConfigManager::new(path.clone());
        let config = UserConfig::default();

        manager.save(&config).unwrap();
        assert!(path.exists());
    }
}
