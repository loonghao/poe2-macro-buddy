use crate::config::Config;
use crate::macro_engine::{MacroEngineState, MacroStatus};
use tauri::State;

const CONFIG_PATH: &str = "config.yaml";

#[tauri::command]
pub async fn load_config() -> Result<Config, String> {
    Config::load_or_default(CONFIG_PATH).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
    config.save(CONFIG_PATH).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_config(config: Config) -> Result<Option<String>, String> {
    match config.validate() {
        Ok(_) => Ok(None),
        Err(e) => Ok(Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn start_macro_engine(state: State<'_, MacroEngineState>) -> Result<(), String> {
    let config = Config::load_or_default(CONFIG_PATH).map_err(|e| e.to_string())?;
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
