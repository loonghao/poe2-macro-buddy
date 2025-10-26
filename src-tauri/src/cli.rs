use anyhow::Result;
use std::env;
use tracing::info;

use crate::config::Config;
use crate::macro_engine::MacroEngine;

/// Run the CLI version of the macro tool
pub async fn run_cli() -> Result<()> {
    info!("POE2 Macro Buddy - CLI Mode");
    info!("Tip: Run without 'cli' argument to launch the GUI");

    // Load or create default configuration
    let config_path = "config.yaml";
    let config = Config::load_or_default(config_path)?;

    info!("Configuration loaded from: {}", config_path);

    // Create and run macro engine
    let mut engine = MacroEngine::new(config);
    engine.run().await?;

    Ok(())
}

/// Check if CLI mode is requested
pub fn is_cli_mode() -> bool {
    let args: Vec<String> = env::args().collect();
    args.len() > 1 && args[1].to_lowercase() == "cli"
}

