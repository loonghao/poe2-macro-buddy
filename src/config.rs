use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuration for a single key macro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMacro {
    /// Key to press (e.g., "1", "2", "e", etc.)
    pub key: String,

    /// Base interval between key presses in milliseconds
    pub interval_ms: u64,

    /// Random variance in milliseconds (±random_variance_ms)
    /// For example, if interval_ms=1000 and random_variance_ms=200,
    /// actual interval will be between 800-1200ms
    #[serde(default)]
    pub random_variance_ms: u64,

    /// Hotkey to toggle this specific macro on/off (e.g., "F1", "F2", etc.)
    pub toggle_hotkey: String,

    /// Whether this macro is enabled by default
    #[serde(default)]
    pub enabled_by_default: bool,
}

/// Configuration for macro behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// List of key macros to run
    pub macros: Vec<KeyMacro>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            macros: vec![
                KeyMacro {
                    key: "1".to_string(),
                    interval_ms: 1000,
                    random_variance_ms: 200,
                    toggle_hotkey: "F9".to_string(),
                    enabled_by_default: false,
                },
                KeyMacro {
                    key: "e".to_string(),
                    interval_ms: 1500,
                    random_variance_ms: 300,
                    toggle_hotkey: "F10".to_string(),
                    enabled_by_default: false,
                },
                KeyMacro {
                    key: "2".to_string(),
                    interval_ms: 2000,
                    random_variance_ms: 400,
                    toggle_hotkey: "F11".to_string(),
                    enabled_by_default: false,
                },
            ],
        }
    }
}

impl Config {
    /// Load configuration from YAML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to YAML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    /// Load config or create default if not exists
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load(path)
        } else {
            let config = Self::default();
            config.save(&path)?;
            Ok(config)
        }
    }
}
