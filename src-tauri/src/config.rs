use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Type of action for a macro
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Keyboard,
    Mouse,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Keyboard
    }
}

/// Mouse button type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Configuration for a single macro (keyboard or mouse)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMacro {
    /// Type of action (keyboard or mouse)
    #[serde(default)]
    pub action_type: ActionType,

    /// Key to press (e.g., "1", "2", "e", etc.) - used when action_type is Keyboard
    #[serde(default)]
    pub key: String,

    /// Mouse button to click - used when action_type is Mouse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_button: Option<MouseButton>,

    /// Base interval between actions in milliseconds
    pub interval_ms: u64,

    /// Random variance in milliseconds (±random_variance_ms)
    #[serde(default)]
    pub random_variance_ms: u64,

    /// Hotkey to toggle this specific macro on/off
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
                    action_type: ActionType::Keyboard,
                    key: "1".to_string(),
                    mouse_button: None,
                    interval_ms: 1000,
                    random_variance_ms: 200,
                    toggle_hotkey: "F9".to_string(),
                    enabled_by_default: false,
                },
                KeyMacro {
                    action_type: ActionType::Keyboard,
                    key: "e".to_string(),
                    mouse_button: None,
                    interval_ms: 1500,
                    random_variance_ms: 300,
                    toggle_hotkey: "F10".to_string(),
                    enabled_by_default: false,
                },
                KeyMacro {
                    action_type: ActionType::Mouse,
                    key: String::new(),
                    mouse_button: Some(MouseButton::Left),
                    interval_ms: 800,
                    random_variance_ms: 150,
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

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.macros.is_empty() {
            return Err(anyhow::anyhow!("No macros configured"));
        }

        // Check for valid action configuration
        for (idx, macro_config) in self.macros.iter().enumerate() {
            match macro_config.action_type {
                ActionType::Keyboard => {
                    if macro_config.key.is_empty() {
                        return Err(anyhow::anyhow!("Macro #{} has empty key", idx));
                    }
                }
                ActionType::Mouse => {
                    if macro_config.mouse_button.is_none() {
                        return Err(anyhow::anyhow!(
                            "Macro #{} has no mouse button specified",
                            idx
                        ));
                    }
                }
            }
        }

        // Check for duplicate keys (only for keyboard macros)
        let mut keys = std::collections::HashSet::new();
        for macro_config in &self.macros {
            if macro_config.action_type == ActionType::Keyboard && !macro_config.key.is_empty() {
                if !keys.insert(&macro_config.key) {
                    return Err(anyhow::anyhow!("Duplicate key: {}", macro_config.key));
                }
            }
        }

        // Check for duplicate hotkeys
        let mut hotkeys = std::collections::HashSet::new();
        for macro_config in &self.macros {
            if macro_config.toggle_hotkey.is_empty() {
                return Err(anyhow::anyhow!("Macro has empty hotkey"));
            }
            if !hotkeys.insert(&macro_config.toggle_hotkey) {
                return Err(anyhow::anyhow!(
                    "Duplicate hotkey: {}",
                    macro_config.toggle_hotkey
                ));
            }
        }

        Ok(())
    }
}
