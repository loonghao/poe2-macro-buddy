# POE2 宏助手

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Crates.io](https://img.shields.io/crates/v/poe2-macro-buddy.svg)](https://crates.io/crates/poe2-macro-buddy)

[English Documentation](README.md)

一个专为《流放之路2》(Path of Exile 2)设计的合规低频率键盘宏工具，单一可执行文件支持 **CLI** 和 **GUI** 双模式，完全遵循游戏的宏使用规则。

## 使用模式

- **GUI 模式**（默认）：`poe2-macro-buddy.exe` - 现代化 Tauri 界面，支持可视化键盘配置
- **CLI 模式**：`poe2-macro-buddy.exe cli` - 轻量级命令行模式，适合高级用户

## 功能特性

- ✅ **符合POE2规则**: 遵循1:1规则（一次按键 = 一次服务器操作）
- 🖥️ **双界面**: 可选择CLI或现代化GUI界面，支持可视化键盘配置
- 🎯 **多按键支持**: 同时运行多个独立的按键宏（如1、E、2键）
- 🎲 **随机间隔**: 每次按键间隔带随机性，避免固定模式被检测
- ⚡ **低频率**: 可配置的按键间隔，避免被检测
- 🎮 **独立控制**: 每个按键有独立的热键切换（F9、F10、F11等）
- ⚙️ **可视化配置**: 交互式键盘布局，轻松设置（GUI模式）
- 📊 **实时监控**: 实时状态显示和控制（GUI模式）
- 🔥 **热重载**: 配置文件变化自动检测并应用
- 💾 **自动保存**: 配置修改后自动保存（1秒防抖）
- 🔒 **运行时保护**: 引擎运行时禁用配置编辑
- 🔒 **安全**: 不修改游戏内存，仅模拟按键

## POE2 宏使用规则

根据[POE官方论坛指南](https://www.pathofexile.com/forum/view-thread/2077975)：

- ✅ **允许**: 一次按键触发一次服务器端操作
- ❌ **禁止**: 一次按键触发多个游戏操作（自动化游戏玩法）
- ✅ **允许**: 低频率的辅助功能（物品过滤、聊天宏等）

本工具通过提供简单的1:1按键自动化来遵守这些规则。

## 安装

### 前置要求

**CLI 模式：**
- Rust 1.70 或更高版本
- Windows/Linux/macOS

**GUI 模式（额外要求）：**
- Node.js 18.x 或更高版本
- npm 9.x 或更高版本

### 快速开始 - 使用 Just（推荐）

我们使用 [`just`](https://github.com/casey/just) 作为命令运行器 - 完美支持 Windows、Linux 和 macOS！

#### 步骤 1：安装 Just

```bash
# 使用 cargo（推荐）
cargo install just

# 或使用包管理器：
# Windows: scoop install just
# macOS: brew install just
```

#### 步骤 2：构建和运行

```bash
# 克隆仓库
git clone https://github.com/loonghao/poe2-macro-buddy.git
cd poe2-macro-buddy

# 安装所有依赖
just install

# 运行 GUI 开发模式（支持热重载）
just dev

# 或构建生产版本
just build
```

就这么简单！🎉

#### 可用命令

```bash
just --list              # 显示所有可用命令
just help                # 显示帮助信息

# 常用命令
just install             # 安装所有依赖
just dev                 # 运行 GUI 开发模式
just dev-cli             # 运行 CLI 开发模式
just build               # 构建 CLI 和 GUI
just test                # 运行所有测试
just ci                  # 运行所有 CI 检查
just clean               # 清理构建产物
```

#### 其他方法

如果不想使用 `just`，我们还提供：

- **PowerShell 脚本**（Windows）：`.\build.ps1 <command>`
- **Bash 脚本**（Linux/macOS）：`./build.sh <command>`
- **Makefile**：`make <target>`
- **cargo-make**：`cargo make <task>`

所有方法都支持相同的命令（install、dev、build、test 等）。

**输出位置：**
- **可执行文件**：`src-tauri/target/release/poe2-macro-buddy`（Windows 上为 `.exe`）
- **安装包**：`src-tauri/target/release/bundle/`
  - Windows：`.msi` 安装包
  - Linux：`.deb`、`.AppImage`
  - macOS：`.dmg`

## 使用方法

### GUI 模式（推荐）

1. **启动应用程序**
   ```bash
   # 开发模式
   cd ui && npm run tauri dev

   # 或运行构建好的 GUI 可执行文件
   ./src-tauri/target/release/poe2-macro-buddy
   ```

2. **配置宏**
   - 点击"Add New Macro"创建宏
   - 选择动作类型：键盘或鼠标
   - 键盘模式：点击可视化键盘选择目标按键
   - 鼠标模式：点击鼠标图示选择按钮（左键/右键/中键）
   - 点击功能键（F1-F12）设置热键
   - 使用滑块调整间隔和随机偏差
   - 配置会在停止输入 1 秒后**自动保存**
   - **注意**：引擎运行时配置编辑被禁用

3. **运行宏**
   - 点击"Start Engine"开始
   - 使用热键（F9、F10等）切换单个宏
   - 实时监控状态
   - 完成后点击"Stop Engine"

4. **配置热重载**
   - 对 `config.yaml` 的任何修改都会被自动检测
   - 配置会自动重载，无需重启应用
   - 前端 UI 自动更新以反映变化
   - 配置重载时会显示提示通知

### CLI 模式

要使用 CLI 模式，运行可执行文件时添加 `cli` 参数：

```bash
# Windows
poe2-macro-buddy.exe cli

# Linux/macOS
./poe2-macro-buddy cli
```

### 1. 配置

复制示例配置文件：

```bash
cp config.example.yaml config.yaml
```

编辑 `config.yaml`：

```yaml
# 配置多个宏
macros:
  # 键盘宏：按键1，每1秒±200ms按一次
  - action_type: keyboard
    key: "1"
    interval_ms: 1000
    random_variance_ms: 200
    toggle_hotkey: "F9"
    enabled_by_default: false

  # 鼠标宏：左键点击，每1.5秒±300ms点一次
  - action_type: mouse
    mouse_button: left
    interval_ms: 1500
    random_variance_ms: 300
    toggle_hotkey: "F10"
    enabled_by_default: false

  # 键盘宏：按键2，每2秒±400ms按一次
  - action_type: keyboard
    key: "2"
    interval_ms: 2000
    random_variance_ms: 400
    toggle_hotkey: "F11"
    enabled_by_default: false
```

### 2. 运行

```bash
# 以 CLI 模式运行
cd src-tauri
cargo run --release -- cli

# 或运行编译好的二进制文件（CLI 模式）
./src-tauri/target/release/poe2-macro-buddy cli
```

### 3. 控制

每个宏都有独立的热键控制：
- 按 **F9** 切换第一个宏
- 按 **F10** 切换第二个宏
- 按 **F11** 切换第三个宏
- 💡 **笔记本用户注意**: 大多数笔记本需要按 `Fn + F9`、`Fn + F10` 等
- 控制台会显示每个宏的当前状态
- 按 **Ctrl+C** 退出程序

## 配置选项

每个宏支持以下配置：

| 选项 | 类型 | 示例 | 说明 |
|------|------|------|------|
| `action_type` | 字符串 | "keyboard" | 动作类型："keyboard"（键盘）或 "mouse"（鼠标） |
| `key` | 字符串 | "1" | 要按下的键（用于键盘动作） |
| `mouse_button` | 字符串 | "left" | 鼠标按钮："left"（左键）、"right"（右键）或 "middle"（中键）（用于鼠标动作） |
| `interval_ms` | 数字 | 1000 | 基础动作间隔（毫秒） |
| `random_variance_ms` | 数字 | 200 | 随机偏差（±毫秒），0表示无随机性 |
| `toggle_hotkey` | 字符串 | "F9" | 切换此宏的热键（F1-F12） |
| `enabled_by_default` | 布尔值 | false | 启动时是否启用此宏 |

**随机间隔示例：**
- `interval_ms: 1000, random_variance_ms: 200` → 实际间隔：800-1200ms
- `interval_ms: 1500, random_variance_ms: 300` → 实际间隔：1200-1800ms
- `interval_ms: 2000, random_variance_ms: 0` → 实际间隔：固定2000ms

**配置热重载：**
- 对 `config.yaml` 的修改会被自动检测（500ms 防抖）
- 配置会自动重载，无需重启应用
- 运行中的宏会自动使用新设置重启
- GUI 模式下修改会在 1 秒后自动保存

## 支持的按键和动作

### 动作类型
- **键盘**：模拟键盘按键
- **鼠标**：模拟鼠标按钮点击

### 目标按键（键盘动作）
- 数字键：`1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `0`
- 字母键：`q`, `w`, `e`, `r`, `t`, `y`, `u`, `i`, `o`, `p`, `a`, `s`, `d`, `f`, `g`, `h`, `j`, `k`, `l`, `z`, `x`, `c`, `v`, `b`, `n`, `m`
- 功能键：`F1` 到 `F12`

### 鼠标按钮（鼠标动作）
- `left`：鼠标左键
- `right`：鼠标右键
- `middle`：鼠标中键（滚轮点击）

### 切换热键
- 功能键：`F1` 到 `F12`

## 安全性与免责声明

⚠️ **重要提示**:

1. 本工具仅模拟键盘输入，不修改游戏内存
2. 使用风险自负 - 虽然设计符合规则，但GGG的检测系统可能会更新
3. 请始终遵守POE2官方服务条款
4. 作者不对任何账号处罚负责

## 开发

### 项目结构

```
poe2-macro-buddy/
├── src/
│   ├── main.rs           # 入口点
│   ├── lib.rs            # 库导出
│   ├── config.rs         # 配置处理
│   └── macro_engine.rs   # 核心宏逻辑
├── src-tauri/            # Tauri GUI 应用
├── ui/                   # React 前端
├── config.example.yaml   # 示例配置
├── Cargo.toml           # 依赖项
└── README.md            # 说明文档
```

### 依赖项

**后端（Rust）：**
- `enigo` - 跨平台输入模拟
- `device_query` - 键盘状态检测
- `tokio` - 异步运行时
- `serde` - 序列化
- `serde_yaml` - YAML配置
- `tracing` - 日志记录
- `notify` - 文件系统事件监控，用于热重载
- `tauri` - 桌面应用框架

**前端（TypeScript/React）：**
- `react` - UI 框架
- `vite` - 构建工具和开发服务器
- `shadcn/ui` - UI 组件库
- `tailwindcss` - 实用优先的 CSS 框架

## 贡献

欢迎贡献！请随时提交Pull Request。

## 许可证

本项目采用MIT许可证 - 详见LICENSE文件。

## 常见问题

**问：笔记本上F9热键不生效？**
答：大多数笔记本需要按 `Fn + F9` 来触发功能键。

**问：可以使用支持列表以外的其他键吗？**
答：GUI 模式支持所有字母数字键（0-9、a-z）和功能键（F1-F12）。你还可以配置鼠标按钮点击。

**问：使用这个工具安全吗？**
答：此工具仅模拟键盘/鼠标输入，不修改游戏内存。但请自行承担风险，并始终遵守POE2的服务条款。

**问：需要手动保存配置修改吗？**
答：不需要！在 GUI 模式下，配置修改会在停止输入 1 秒后自动保存。你也可以直接编辑 `config.yaml`，修改会被自动检测并应用。

**问：引擎运行时可以编辑配置吗？**
答：不可以，引擎运行时配置编辑被禁用以防止运行时错误。请先停止引擎再进行修改。

**问：热重载是如何工作的？**
答：应用程序使用文件监听器监控 `config.yaml` 的变化。当检测到变化时（带 500ms 防抖），配置会自动重载，运行中的宏会使用新设置重启。

## 致谢

- [enigo](https://github.com/enigo-rs/enigo) - 输入模拟库
- [device_query](https://github.com/ostrosco/device_query) - 设备状态库
- [Tauri](https://tauri.app/) - 桌面应用框架
- [shadcn/ui](https://ui.shadcn.com/) - UI 组件库
- POE2社区提供的宏使用指南

