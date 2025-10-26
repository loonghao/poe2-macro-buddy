use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Button, Enigo, Key, Keyboard, Mouse, Settings};
use rand::Rng;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use tracing::{info, warn};

use crate::config::{ActionType, Config, KeyMacro, MouseButton};

#[derive(Debug, Clone, Serialize)]
pub struct MacroStatus {
    pub index: usize,
    pub enabled: bool,
    pub action_type: ActionType,
    pub key: String,
    pub mouse_button: Option<MouseButton>,
    pub toggle_hotkey: String,
}

/// Macro engine state
pub struct MacroEngineState {
    config: Arc<RwLock<Option<Config>>>,
    running: Arc<AtomicBool>,
    macro_states: Arc<RwLock<Vec<Arc<AtomicBool>>>>,
}

impl MacroEngineState {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
            running: Arc::new(AtomicBool::new(false)),
            macro_states: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start(&self, config: Config) -> Result<()> {
        if self.running.load(Ordering::Relaxed) {
            return Err(anyhow::anyhow!("Macro engine is already running"));
        }

        config.validate()?;

        info!(
            "Starting POE2 Macro Engine with {} macro(s)...",
            config.macros.len()
        );

        // Initialize macro states
        let states: Vec<Arc<AtomicBool>> = config
            .macros
            .iter()
            .map(|m| Arc::new(AtomicBool::new(m.enabled_by_default)))
            .collect();

        *self.macro_states.write().await = states.clone();
        *self.config.write().await = Some(config.clone());
        self.running.store(true, Ordering::Relaxed);

        // Spawn tasks for each macro
        for (idx, macro_config) in config.macros.iter().enumerate() {
            let macro_config = macro_config.clone();
            let enabled = states[idx].clone();
            let running = self.running.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::run_single_macro(idx, macro_config, enabled, running).await {
                    warn!("Macro {} error: {}", idx, e);
                }
            });
        }

        Ok(())
    }

    pub async fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
        *self.config.write().await = None;
        *self.macro_states.write().await = Vec::new();
        info!("Macro engine stopped");
    }

    pub async fn get_status(&self) -> Vec<MacroStatus> {
        let config_guard = self.config.read().await;
        let states_guard = self.macro_states.read().await;

        if let Some(config) = config_guard.as_ref() {
            config
                .macros
                .iter()
                .enumerate()
                .map(|(idx, macro_config)| MacroStatus {
                    index: idx,
                    enabled: states_guard
                        .get(idx)
                        .map(|s| s.load(Ordering::Relaxed))
                        .unwrap_or(false),
                    action_type: macro_config.action_type.clone(),
                    key: macro_config.key.clone(),
                    mouse_button: macro_config.mouse_button.clone(),
                    toggle_hotkey: macro_config.toggle_hotkey.clone(),
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub async fn toggle_macro(&self, index: usize) -> Result<()> {
        let states = self.macro_states.read().await;
        if let Some(state) = states.get(index) {
            let current = state.load(Ordering::Relaxed);
            state.store(!current, Ordering::Relaxed);
            info!("Macro #{} toggled to {}", index, !current);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid macro index: {}", index))
        }
    }

    async fn run_single_macro(
        idx: usize,
        macro_config: KeyMacro,
        enabled: Arc<AtomicBool>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        // Log macro configuration
        match macro_config.action_type {
            ActionType::Keyboard => {
                info!(
                    "Macro #{}: Type=Keyboard, Key='{}', Interval={}ms±{}ms, Toggle='{}'",
                    idx,
                    macro_config.key,
                    macro_config.interval_ms,
                    macro_config.random_variance_ms,
                    macro_config.toggle_hotkey
                );
            }
            ActionType::Mouse => {
                let button_name = match macro_config.mouse_button {
                    Some(MouseButton::Left) => "Left",
                    Some(MouseButton::Right) => "Right",
                    Some(MouseButton::Middle) => "Middle",
                    None => "Unknown",
                };
                info!(
                    "Macro #{}: Type=Mouse, Button='{}', Interval={}ms±{}ms, Toggle='{}'",
                    idx,
                    button_name,
                    macro_config.interval_ms,
                    macro_config.random_variance_ms,
                    macro_config.toggle_hotkey
                );
            }
        }

        // Parse key or mouse button based on action type
        let target_key = if macro_config.action_type == ActionType::Keyboard {
            Some(
                Self::str_to_key(&macro_config.key)
                    .ok_or_else(|| anyhow::anyhow!("Invalid key: {}", macro_config.key))?,
            )
        } else {
            None
        };

        let target_mouse_button = if macro_config.action_type == ActionType::Mouse {
            macro_config.mouse_button.as_ref().map(|btn| match btn {
                MouseButton::Left => Button::Left,
                MouseButton::Right => Button::Right,
                MouseButton::Middle => Button::Middle,
            })
        } else {
            None
        };

        // Parse toggle hotkey
        let toggle_keycode = Self::str_to_keycode(&macro_config.toggle_hotkey);

        // Spawn hotkey detection task
        let enabled_clone = enabled.clone();
        let running_clone = running.clone();
        let toggle_hotkey = macro_config.toggle_hotkey.clone();
        let device_state = DeviceState::new();

        tokio::spawn(async move {
            let mut last_toggle_state = false;
            // Optimized: Increase interval to reduce CPU usage (100ms is still responsive)
            let mut check_interval = time::interval(Duration::from_millis(100));

            while running_clone.load(Ordering::Relaxed) {
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

        // Main action loop (keyboard or mouse)
        let mut enigo = Enigo::new(&Settings::default())?;

        // Pre-compute button name for logging (optimization)
        let button_name = match macro_config.mouse_button {
            Some(MouseButton::Left) => "Left",
            Some(MouseButton::Right) => "Right",
            Some(MouseButton::Middle) => "Middle",
            None => "Unknown",
        };

        while running.load(Ordering::Relaxed) {
            let interval =
                Self::calculate_interval(macro_config.interval_ms, macro_config.random_variance_ms);

            time::sleep(interval).await;

            if enabled.load(Ordering::Relaxed) {
                match macro_config.action_type {
                    ActionType::Keyboard => {
                        if let Some(key) = target_key {
                            match enigo.key(key, enigo::Direction::Click) {
                                Ok(_) => {
                                    tracing::debug!(
                                        "Macro #{}: Pressed key '{}' (next in ~{}ms)",
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
                    ActionType::Mouse => {
                        if let Some(button) = target_mouse_button {
                            match enigo.button(button, enigo::Direction::Click) {
                                Ok(_) => {
                                    tracing::debug!(
                                        "Macro #{}: Clicked mouse button '{}' (next in ~{}ms)",
                                        idx,
                                        button_name,
                                        interval.as_millis()
                                    );
                                }
                                Err(e) => {
                                    warn!("Macro #{}: Failed to click mouse button: {}", idx, e);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

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

    fn calculate_interval(base_ms: u64, variance_ms: u64) -> Duration {
        if variance_ms == 0 {
            return Duration::from_millis(base_ms);
        }

        let mut rng = rand::thread_rng();
        let variance = rng.gen_range(-(variance_ms as i64)..=(variance_ms as i64));
        let actual_ms = (base_ms as i64 + variance).max(100) as u64;
        Duration::from_millis(actual_ms)
    }
}

/// Simple macro engine for CLI mode
pub struct MacroEngine {
    config: Config,
}

impl MacroEngine {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

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
        let running = Arc::new(AtomicBool::new(true));

        for (idx, macro_config) in self.config.macros.iter().enumerate() {
            let macro_config = macro_config.clone();
            let enabled = Arc::new(AtomicBool::new(macro_config.enabled_by_default));
            let running_clone = running.clone();

            let handle = tokio::spawn(async move {
                if let Err(e) =
                    MacroEngineState::run_single_macro(idx, macro_config, enabled, running_clone)
                        .await
                {
                    warn!("Macro {} error: {}", idx, e);
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
}
