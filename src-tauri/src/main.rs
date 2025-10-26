// Note: We don't use windows_subsystem = "windows" because we need console for CLI mode
// The GUI will still work fine without it

mod cli;
mod commands;
mod config;
mod macro_engine;

use commands::ConfigCache;
use macro_engine::MacroEngineState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Check if CLI mode is requested
    if cli::is_cli_mode() {
        // Run CLI mode
        if let Err(e) = cli::run_cli().await {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Run GUI mode
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(MacroEngineState::new())
        .manage(ConfigCache::new())
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
