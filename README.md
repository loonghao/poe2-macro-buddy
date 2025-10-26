# POE2 Macro Buddy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Crates.io](https://img.shields.io/crates/v/poe2-macro-buddy.svg)](https://crates.io/crates/poe2-macro-buddy)

[中文文档](README_zh.md)

A compliant low-frequency keyboard macro tool designed for Path of Exile 2, with both **CLI** and **GUI** modes in a single executable, following the game's macro usage rules.

## Usage Modes

- **GUI Mode** (default): `poe2-macro-buddy.exe` - Modern Tauri-based interface with visual keyboard configuration
- **CLI Mode**: `poe2-macro-buddy.exe cli` - Lightweight command-line mode for advanced users

## Features

- ✅ **POE2 Compliant**: Follows the 1:1 rule (one key press = one server action)
- 🖥️ **Dual Interface**: Choose between CLI or modern GUI with visual keyboard configuration
- 🎯 **Multiple Keys**: Run multiple independent key macros simultaneously (e.g., 1, E, 2)
- 🎲 **Random Intervals**: Each key press has randomized timing to avoid pattern detection
- ⚡ **Low Frequency**: Configurable interval to avoid detection
- 🎮 **Independent Control**: Each key has its own toggle hotkey (F9, F10, F11, etc.)
- ⚙️ **Visual Configuration**: Interactive keyboard layout for easy setup (GUI mode)
- 📊 **Real-time Monitoring**: Live status display and control (GUI mode)
- 🔥 **Hot Reload**: Configuration changes are automatically detected and applied
- 💾 **Auto-save**: Configuration changes are automatically saved (1-second debounce)
- 🔒 **Runtime Protection**: Configuration editing is disabled while engine is running
- 🔒 **Safe**: No game memory manipulation, only simulates key presses

## POE2 Macro Rules

According to [official POE forum guidelines](https://www.pathofexile.com/forum/view-thread/2077975):

- ✅ **Allowed**: One key press triggers one server-side action
- ❌ **Forbidden**: One key press triggers multiple game actions (automation)
- ✅ **Allowed**: Low-frequency assistance features (item filters, chat macros)

This tool is designed to comply with these rules by providing a simple 1:1 key press automation.

## Installation

### Prerequisites

**For CLI Mode:**
- Rust 1.70 or higher
- Windows/Linux/macOS

**For GUI Mode (Additional):**
- Node.js 18.x or higher
- npm 9.x or higher

### Quick Start - Using Just (Recommended)

We use [`just`](https://github.com/casey/just) as our command runner - it works perfectly on Windows, Linux, and macOS!

#### Step 1: Install Just

```bash
# Using cargo (recommended)
cargo install just

# Or using package managers:
# Windows: scoop install just
# macOS: brew install just
```

#### Step 2: Build and Run

```bash
# Clone the repository
git clone https://github.com/loonghao/poe2-macro-buddy.git
cd poe2-macro-buddy

# Install all dependencies
just install

# Run GUI in development mode (with hot-reload)
just dev

# Or build everything for production
just build
```

That's it! 🎉

#### Available Commands

```bash
just --list              # Show all available commands
just help                # Show help with descriptions

# Common commands
just install             # Install all dependencies
just dev                 # Run GUI in development mode
just dev-cli             # Run CLI in development mode
just build               # Build both CLI and GUI
just test                # Run all tests
just ci                  # Run all CI checks
just clean               # Clean build artifacts
```

#### Alternative Methods

If you prefer not to use `just`, we also provide:

- **PowerShell Script** (Windows): `.\build.ps1 <command>`
- **Bash Script** (Linux/macOS): `./build.sh <command>`
- **Makefile**: `make <target>`
- **cargo-make**: `cargo make <task>`

All methods support the same commands (install, dev, build, test, etc.).

**Output Locations:**
- **Executable**: `src-tauri/target/release/poe2-macro-buddy` (or `.exe` on Windows)
- **Installers**: `src-tauri/target/release/bundle/`
  - Windows: `.msi` installer
  - Linux: `.deb`, `.AppImage`
  - macOS: `.dmg`

## Usage

### GUI Mode (Recommended)

1. **Launch the Application**
   ```bash
   # Development
   cd ui && npm run tauri dev

   # Or run the built GUI executable
   ./src-tauri/target/release/poe2-macro-buddy
   ```

2. **Configure Macros**
   - Click "Add New Macro" to create a macro
   - Select action type: Keyboard or Mouse
   - For Keyboard: Click on the visual keyboard to select target key
   - For Mouse: Click on the mouse diagram to select button (left/right/middle)
   - Click on function keys (F1-F12) to set hotkey
   - Adjust interval and variance with sliders
   - Configuration is **automatically saved** after 1 second of inactivity
   - **Note**: Configuration editing is disabled while engine is running

3. **Run Macros**
   - Click "Start Engine" to begin
   - Use hotkeys (F9, F10, etc.) to toggle individual macros
   - Monitor status in real-time
   - Click "Stop Engine" when done

4. **Configuration Hot Reload**
   - Any changes to `config.yaml` are automatically detected
   - Configuration is reloaded without restarting the application
   - Frontend UI updates automatically to reflect changes
   - A toast notification appears when configuration is reloaded

### CLI Mode

To use CLI mode, run the executable with the `cli` argument:

```bash
# Windows
poe2-macro-buddy.exe cli

# Linux/macOS
./poe2-macro-buddy cli
```

### 1. Configure

Copy the example configuration:

```bash
cp config.example.yaml config.yaml
```

Edit `config.yaml`:

```yaml
# Configure multiple key macros
macros:
  # Key 1: Press every 1s ±200ms
  - key: "1"
    interval_ms: 1000
    random_variance_ms: 200
    toggle_hotkey: "F9"
    enabled_by_default: false

  # Key E: Press every 1.5s ±300ms
  - key: "e"
    interval_ms: 1500
    random_variance_ms: 300
    toggle_hotkey: "F10"
    enabled_by_default: false

  # Key 2: Press every 2s ±400ms
  - key: "2"
    interval_ms: 2000
    random_variance_ms: 400
    toggle_hotkey: "F11"
    enabled_by_default: false
```

### 2. Run

```bash
# Run in CLI mode
cd src-tauri
cargo run --release -- cli

# Or run the compiled binary in CLI mode
./src-tauri/target/release/poe2-macro-buddy cli
```

### 3. Control

Each macro has independent hotkey control:
- Press **F9** to toggle key "1" macro
- Press **F10** to toggle key "E" macro
- Press **F11** to toggle key "2" macro
- 💡 **Laptop users**: Most laptops require `Fn + F9`, `Fn + F10`, etc.
- The console will show the current state of each macro
- Press **Ctrl+C** to exit

## Configuration Options

Each macro supports the following options:

| Option | Type | Example | Description |
|--------|------|---------|-------------|
| `action_type` | string | "keyboard" | Action type: "keyboard" or "mouse" |
| `key` | string | "1" | Key to press (for keyboard actions) |
| `mouse_button` | string | "left" | Mouse button: "left", "right", or "middle" (for mouse actions) |
| `interval_ms` | number | 1000 | Base interval between actions (milliseconds) |
| `random_variance_ms` | number | 200 | Random variance (±milliseconds), 0 for no randomness |
| `toggle_hotkey` | string | "F9" | Hotkey to toggle this macro (F1-F12) |
| `enabled_by_default` | boolean | false | Whether this macro starts enabled |

**Random Interval Examples:**
- `interval_ms: 1000, random_variance_ms: 200` → Actual: 800-1200ms
- `interval_ms: 1500, random_variance_ms: 300` → Actual: 1200-1800ms
- `interval_ms: 2000, random_variance_ms: 0` → Actual: Fixed 2000ms

**Configuration Hot Reload:**
- Changes to `config.yaml` are automatically detected (500ms debounce)
- Configuration is reloaded without restarting the application
- Running macros are automatically restarted with new settings
- GUI auto-saves changes after 1 second of inactivity

## Supported Keys and Actions

### Action Types
- **Keyboard**: Simulates keyboard key presses
- **Mouse**: Simulates mouse button clicks

### Target Keys (Keyboard Actions)
- Number keys: `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `0`
- Letter keys: `q`, `w`, `e`, `r`, `t`, `y`, `u`, `i`, `o`, `p`, `a`, `s`, `d`, `f`, `g`, `h`, `j`, `k`, `l`, `z`, `x`, `c`, `v`, `b`, `n`, `m`
- Function keys: `F1` through `F12`

### Mouse Buttons (Mouse Actions)
- `left`: Left mouse button
- `right`: Right mouse button
- `middle`: Middle mouse button (scroll wheel click)

### Toggle Hotkeys
- Function keys: `F1` through `F12`

## Safety & Disclaimer

⚠️ **Important Notes**:

1. This tool only simulates keyboard input and does not modify game memory
2. Use at your own risk - while designed to be compliant, GGG's detection systems may evolve
3. Always follow the official POE2 Terms of Service
4. The author is not responsible for any account actions

## Development

### Project Structure

```
poe2-macro-buddy/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── config.rs         # Configuration handling
│   └── macro_engine.rs   # Core macro logic
├── src-tauri/            # Tauri GUI application
├── ui/                   # React frontend
├── config.example.yaml   # Example configuration
├── Cargo.toml           # Dependencies
└── README.md            # This file
```

### Dependencies

**Backend (Rust):**
- `enigo` - Cross-platform input simulation
- `device_query` - Keyboard state detection
- `tokio` - Async runtime
- `serde` - Serialization
- `serde_yaml` - YAML configuration
- `tracing` - Logging
- `notify` - File system event monitoring for hot reload
- `tauri` - Desktop application framework

**Frontend (TypeScript/React):**
- `react` - UI framework
- `vite` - Build tool and dev server
- `shadcn/ui` - UI component library
- `tailwindcss` - Utility-first CSS framework

## FAQ

**Q: F9 hotkey not working on my laptop?**
A: Most laptops require pressing `Fn + F9` to trigger function keys.

**Q: Can I use other keys besides the supported ones?**
A: The GUI supports all alphanumeric keys (0-9, a-z) and function keys (F1-F12). You can also configure mouse button clicks.

**Q: Is this safe to use?**
A: This tool only simulates keyboard/mouse input and doesn't modify game memory. However, use at your own risk and always follow POE2's Terms of Service.

**Q: Do I need to manually save configuration changes?**
A: No! In GUI mode, configuration changes are automatically saved after 1 second of inactivity. You can also edit `config.yaml` directly, and changes will be automatically detected and applied.

**Q: Can I edit configuration while the engine is running?**
A: No, configuration editing is disabled while the engine is running to prevent runtime errors. Stop the engine first to make changes.

**Q: How does hot reload work?**
A: The application monitors `config.yaml` for changes using a file watcher. When changes are detected (with 500ms debounce), the configuration is automatically reloaded and running macros are restarted with new settings.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [enigo](https://github.com/enigo-rs/enigo) - Input simulation library
- [device_query](https://github.com/ostrosco/device_query) - Device state library
- [Tauri](https://tauri.app/) - Desktop application framework
- [shadcn/ui](https://ui.shadcn.com/) - UI component library
- POE2 community for macro usage guidelines

