use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, Key, Keyboard, Settings};
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};

use crate::config::Config;

/// Macro engine that handles key press automation
pub struct MacroEngine {
    config: Config,
}

impl MacroEngine {
    /// Create a new macro engine with given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Convert hotkey string to Keycode
    fn str_to_keycode(hotkey: &str) -> Option<Keycode> {
        match hotkey {
            "F1" => Some(Keycode::F1),
            "F2" => Some(Keycode::F2),
            "F3" => Some(Keycode::F3),
            "F4" => Some(Keycode::F4),
            "F5" => Some(Keycode::F5),
            "F6" => Some(Keycode::F6),
            "F7" => Some(Keycode::F7),
            "F8" => Some(Keycode::F8),
            "F9" => Some(Keycode::F9),
            "F10" => Some(Keycode::F10),
            "F11" => Some(Keycode::F11),
            "F12" => Some(Keycode::F12),
            _ => None,
        }
    }

    /// Convert key string to enigo Key
    fn str_to_key(key_str: &str) -> Option<Key> {
        match key_str {
            "1" => Some(Key::Unicode('1')),
            "2" => Some(Key::Unicode('2')),
            "3" => Some(Key::Unicode('3')),
            "4" => Some(Key::Unicode('4')),
            "5" => Some(Key::Unicode('5')),
            "q" | "Q" => Some(Key::Unicode('q')),
            "w" | "W" => Some(Key::Unicode('w')),
            "e" | "E" => Some(Key::Unicode('e')),
            "r" | "R" => Some(Key::Unicode('r')),
            "t" | "T" => Some(Key::Unicode('t')),
            _ => None,
        }
    }

    /// Calculate interval with random variance
    fn calculate_interval(base_ms: u64, variance_ms: u64) -> Duration {
        if variance_ms == 0 {
            return Duration::from_millis(base_ms);
        }

        let mut rng = rand::thread_rng();
        let variance = rng.gen_range(-(variance_ms as i64)..=(variance_ms as i64));
        let actual_ms = (base_ms as i64 + variance).max(100) as u64; // Minimum 100ms
        Duration::from_millis(actual_ms)
    }

    /// Run the macro engine
    pub async fn run(&mut self) -> Result<()> {
        info!(
            "Starting POE2 Macro Engine with {} macro(s)...",
            self.config.macros.len()
        );
        info!("TIP: On laptops, you may need to press Fn+F# to toggle");

        if self.config.macros.is_empty() {
            return Err(anyhow::anyhow!("No macros configured"));
        }

        // Spawn a task for each macro
        let mut handles = vec![];

        for (idx, macro_config) in self.config.macros.iter().enumerate() {
            let macro_config = macro_config.clone();
            let macro_idx = idx;

            let handle = tokio::spawn(async move {
                if let Err(e) = Self::run_single_macro(macro_idx, macro_config).await {
                    warn!("Macro {} error: {}", macro_idx, e);
                }
            });

            handles.push(handle);
        }

        // Wait for all macros (they run forever)
        for handle in handles {
            let _ = handle.await;
        }

        Ok(())
    }

    /// Run a single macro task
    async fn run_single_macro(idx: usize, macro_config: crate::config::KeyMacro) -> Result<()> {
        info!(
            "Macro #{}: Key='{}', Interval={}ms±{}ms, Toggle='{}'",
            idx,
            macro_config.key,
            macro_config.interval_ms,
            macro_config.random_variance_ms,
            macro_config.toggle_hotkey
        );

        // Parse key
        let target_key = Self::str_to_key(&macro_config.key)
            .ok_or_else(|| anyhow::anyhow!("Invalid key: {}", macro_config.key))?;

        // Parse toggle hotkey
        let toggle_keycode = Self::str_to_keycode(&macro_config.toggle_hotkey);

        // Enabled state for this macro
        let enabled = Arc::new(AtomicBool::new(macro_config.enabled_by_default));

        // Spawn hotkey detection task
        let enabled_clone = enabled.clone();
        let toggle_hotkey = macro_config.toggle_hotkey.clone();
        let device_state = DeviceState::new();

        tokio::spawn(async move {
            let mut last_toggle_state = false;
            let mut check_interval = time::interval(Duration::from_millis(50));

            loop {
                check_interval.tick().await;

                if let Some(toggle_key) = toggle_keycode {
                    let keys = device_state.get_keys();
                    let toggle_pressed = keys.contains(&toggle_key);

                    if toggle_pressed && !last_toggle_state {
                        let current = enabled_clone.load(Ordering::Relaxed);
                        enabled_clone.store(!current, Ordering::Relaxed);

                        if !current {
                            info!(
                                "Macro #{} ENABLED - Press {} to disable",
                                idx, toggle_hotkey
                            );
                        } else {
                            info!(
                                "Macro #{} DISABLED - Press {} to enable",
                                idx, toggle_hotkey
                            );
                        }

                        time::sleep(Duration::from_millis(300)).await;
                    }
                    last_toggle_state = toggle_pressed;
                }
            }
        });

        // Main key pressing loop
        let mut enigo = Enigo::new(&Settings::default())?;

        loop {
            // Calculate next interval with randomness
            let interval =
                Self::calculate_interval(macro_config.interval_ms, macro_config.random_variance_ms);

            time::sleep(interval).await;

            // Press key if enabled
            if enabled.load(Ordering::Relaxed) {
                match enigo.key(target_key, enigo::Direction::Click) {
                    Ok(_) => {
                        tracing::debug!(
                            "Macro #{}: Pressed '{}' (next in ~{}ms)",
                            idx,
                            macro_config.key,
                            interval.as_millis()
                        );
                    }
                    Err(e) => {
                        warn!("Macro #{}: Failed to press key: {}", idx, e);
                    }
                }
            }
        }
    }
}
