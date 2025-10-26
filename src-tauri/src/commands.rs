use crate::config::Config;
use crate::macro_engine::{MacroEngineState, MacroStatus};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

const CONFIG_PATH: &str = "config.yaml";

/// Cached configuration to avoid repeated file reads
pub struct ConfigCache {
    config: Arc<RwLock<Option<Config>>>,
}

impl Default for ConfigCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigCache {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_or_load(&self) -> Result<Config, String> {
        // Try to get from cache first
        {
            let cache = self.config.read().await;
            if let Some(config) = cache.as_ref() {
                return Ok(config.clone());
            }
        }

        // Load from file and cache
        let config = Config::load_or_default(CONFIG_PATH).map_err(|e| e.to_string())?;
        {
            let mut cache = self.config.write().await;
            *cache = Some(config.clone());
        }
        Ok(config)
    }

    pub async fn invalidate(&self) {
        let mut cache = self.config.write().await;
        *cache = None;
    }
}

#[tauri::command]
pub async fn load_config(cache: State<'_, ConfigCache>) -> Result<Config, String> {
    cache.get_or_load().await
}

#[tauri::command]
pub async fn save_config(config: Config, cache: State<'_, ConfigCache>) -> Result<(), String> {
    config.save(CONFIG_PATH).map_err(|e| e.to_string())?;
    // Invalidate cache after save
    cache.invalidate().await;
    Ok(())
}

#[tauri::command]
pub async fn validate_config(config: Config) -> Result<Option<String>, String> {
    match config.validate() {
        Ok(_) => Ok(None),
        Err(e) => Ok(Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn start_macro_engine(
    state: State<'_, MacroEngineState>,
    cache: State<'_, ConfigCache>,
) -> Result<(), String> {
    let config = cache.get_or_load().await?;
    state.start(config).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_macro_engine(state: State<'_, MacroEngineState>) -> Result<(), String> {
    state.stop().await;
    Ok(())
}

#[tauri::command]
pub async fn get_macro_status(
    state: State<'_, MacroEngineState>,
) -> Result<Vec<MacroStatus>, String> {
    Ok(state.get_status().await)
}

#[tauri::command]
pub async fn toggle_macro(index: usize, state: State<'_, MacroEngineState>) -> Result<(), String> {
    state.toggle_macro(index).await.map_err(|e| e.to_string())
}
