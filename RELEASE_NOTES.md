# POE2 Macro Buddy v0.1.0

## 🎉 Initial Release

POE2 Macro Buddy is a compliant low-frequency keyboard and mouse macro tool for Path of Exile 2, featuring both CLI and modern GUI interfaces.

## ✨ Key Features

### Core Functionality
- ✅ **POE2 Compliant**: Follows the 1:1 rule (one key press = one server action)
- 🎯 **Multiple Macros**: Run multiple independent key/mouse macros simultaneously
- 🎲 **Random Intervals**: Each action has randomized timing to avoid pattern detection
- 🎮 **Independent Control**: Each macro has its own toggle hotkey (F1-F12)
- 🔒 **Safe**: No game memory manipulation, only simulates input

### User Interface
- 🖥️ **Dual Interface**: Choose between CLI or modern GUI
- ⚙️ **Visual Configuration**: Interactive keyboard and mouse button selectors
- 📊 **Real-time Monitoring**: Live status display and control
- 🔥 **Hot Reload**: Configuration changes are automatically detected and applied
- 💾 **Auto-save**: Configuration changes are automatically saved (1-second debounce)
- 🔒 **Runtime Protection**: Configuration editing disabled while engine is running

### Supported Actions
- **Keyboard**: All alphanumeric keys (0-9, a-z) and function keys (F1-F12)
- **Mouse**: Left, right, and middle button clicks

## 📦 Installation

### Windows

**Option 1: MSI Installer (Recommended)**
- Download: `POE2 Macro Buddy_0.1.0_x64_en-US.msi` (4.50 MB)
- Double-click to install
- Includes automatic updates and uninstaller

**Option 2: NSIS Installer**
- Download: `POE2 Macro Buddy_0.1.0_x64-setup.exe` (2.84 MB)
- Portable installation option available

### First Run

1. Launch the application
2. Configure your macros using the visual interface
3. Click "Start Engine" to begin
4. Use hotkeys (F9, F10, etc.) to toggle individual macros

## 🚀 Quick Start

### GUI Mode (Recommended)

1. **Add a Macro**
   - Click "Add New Macro"
   - Select action type: Keyboard or Mouse
   - Choose target key/button using visual selectors
   - Set toggle hotkey (F1-F12)
   - Adjust interval and variance

2. **Run Macros**
   - Click "Start Engine"
   - Use hotkeys to toggle individual macros
   - Monitor status in real-time

3. **Configuration**
   - Changes auto-save after 1 second
   - External edits to `config.yaml` are auto-detected
   - Configuration editing disabled while engine running

### CLI Mode

```bash
# Run with CLI mode
poe2-macro-buddy.exe cli
```

## 📝 Configuration Example

```yaml
macros:
  # Keyboard macro: Press key "1" every 1000±200ms
  - action_type: keyboard
    key: "1"
    interval_ms: 1000
    random_variance_ms: 200
    toggle_hotkey: "F9"
    enabled_by_default: false

  # Mouse macro: Left click every 1500±300ms
  - action_type: mouse
    mouse_button: left
    interval_ms: 1500
    random_variance_ms: 300
    toggle_hotkey: "F10"
    enabled_by_default: false
```

## 🔧 Technical Details

### Backend (Rust)
- `enigo` - Cross-platform input simulation
- `device_query` - Keyboard state detection
- `tokio` - Async runtime
- `notify` - File system monitoring for hot reload
- `tauri` - Desktop application framework

### Frontend (TypeScript/React)
- `react` - UI framework
- `vite` - Build tool
- `shadcn/ui` - UI components
- `tailwindcss` - Styling

## ⚠️ Important Notes

- **Laptop Users**: Most laptops require `Fn + F9`, `Fn + F10`, etc. to trigger function keys
- **Safety**: This tool only simulates input and doesn't modify game memory
- **Compliance**: Always follow POE2's Terms of Service
- **Runtime Protection**: Stop the engine before editing configuration

## 🐛 Known Issues

None reported yet. Please report issues on GitHub!

## 📖 Documentation

- [English README](https://github.com/loonghao/poe2-macro-buddy/blob/main/README.md)
- [中文文档](https://github.com/loonghao/poe2-macro-buddy/blob/main/README_zh.md)

## 🙏 Acknowledgments

- [enigo](https://github.com/enigo-rs/enigo) - Input simulation library
- [device_query](https://github.com/ostrosco/device_query) - Device state library
- [Tauri](https://tauri.app/) - Desktop application framework
- [shadcn/ui](https://ui.shadcn.com/) - UI component library
- POE2 community for macro usage guidelines

## 📄 License

MIT License - See [LICENSE](LICENSE) for details

---

**Full Changelog**: https://github.com/loonghao/poe2-macro-buddy/commits/v0.1.0

