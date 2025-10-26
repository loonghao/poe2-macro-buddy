import { invoke } from "@tauri-apps/api/core";

export type ActionType = "keyboard" | "mouse";
export type MouseButton = "left" | "right" | "middle";

export interface KeyMacro {
  action_type: ActionType;
  key: string;
  mouse_button?: MouseButton;
  interval_ms: number;
  random_variance_ms: number;
  toggle_hotkey: string;
  enabled_by_default: boolean;
}

export interface Config {
  macros: KeyMacro[];
}

export interface MacroStatus {
  index: number;
  enabled: boolean;
  action_type: ActionType;
  key: string;
  mouse_button?: MouseButton;
  toggle_hotkey: string;
}

// Load configuration from file
export async function loadConfig(): Promise<Config> {
  return await invoke<Config>("load_config");
}

// Save configuration to file
export async function saveConfig(config: Config): Promise<void> {
  await invoke("save_config", { config });
}

// Start macro engine
export async function startMacroEngine(): Promise<void> {
  await invoke("start_macro_engine");
}

// Stop macro engine
export async function stopMacroEngine(): Promise<void> {
  await invoke("stop_macro_engine");
}

// Get macro status
export async function getMacroStatus(): Promise<MacroStatus[]> {
  return await invoke<MacroStatus[]>("get_macro_status");
}

// Toggle specific macro
export async function toggleMacro(index: number): Promise<void> {
  await invoke("toggle_macro", { index });
}

// Validate configuration
export async function validateConfig(config: Config): Promise<string | null> {
  return await invoke<string | null>("validate_config", { config });
}

