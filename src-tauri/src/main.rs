// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod macro_engine;

use macro_engine::MacroEngineState;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(MacroEngineState::new())
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::validate_config,
            commands::start_macro_engine,
            commands::stop_macro_engine,
            commands::get_macro_status,
            commands::toggle_macro,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
