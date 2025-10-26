mod config;
mod macro_engine;

use anyhow::Result;
use config::Config;
use macro_engine::MacroEngine;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Check for --gui flag
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--gui".to_string()) {
        info!("Starting GUI mode...");
        info!(
            "Please use: cargo tauri dev (for development) or cargo tauri build (for production)"
        );
        info!("Or run the compiled Tauri application directly");
        return Ok(());
    }

    info!("POE2 Macro Tool Starting (CLI Mode)...");
    info!("Tip: Use --gui flag to launch the graphical interface");

    // Load or create default configuration
    let config_path = "config.yaml";
    let config = Config::load_or_default(config_path)?;

    info!("Configuration loaded from: {}", config_path);

    // Create and run macro engine
    let mut engine = MacroEngine::new(config);
    engine.run().await?;

    Ok(())
}
