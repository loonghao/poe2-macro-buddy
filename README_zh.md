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
- 🔒 **安全**: 不修改游戏内存，仅模拟按键

## POE2 宏使用规则

根据[POE官方论坛指南](https://www.pathofexile.com/forum/view-thread/2077975)：

- ✅ **允许**: 一次按键触发一次服务器端操作
- ❌ **禁止**: 一次按键触发多个游戏操作（自动化游戏玩法）
- ✅ **允许**: 低频率的辅助功能（物品过滤、聊天宏等）

本工具通过提供简单的1:1按键自动化来遵守这些规则。

## 安装

### 前置要求

- Rust 1.70 或更高版本
- Windows/Linux/macOS

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/loonghao/poe2-macro-buddy.git
cd poe2-macro-buddy

# 构建项目
cargo build --release

# 可执行文件位于 src-tauri/target/release/poe2-macro-buddy
# 默认启动 GUI，添加 cli 参数启动命令行模式
```

## 使用方法

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
# 配置多个按键宏
macros:
  # 按键1：每1秒±200ms按一次
  - key: "1"
    interval_ms: 1000
    random_variance_ms: 200
    toggle_hotkey: "F9"
    enabled_by_default: false

  # 按键E：每1.5秒±300ms按一次
  - key: "e"
    interval_ms: 1500
    random_variance_ms: 300
    toggle_hotkey: "F10"
    enabled_by_default: false

  # 按键2：每2秒±400ms按一次
  - key: "2"
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
- 按 **F9** 切换按键"1"的宏
- 按 **F10** 切换按键"E"的宏
- 按 **F11** 切换按键"2"的宏
- 💡 **笔记本用户注意**: 大多数笔记本需要按 `Fn + F9`、`Fn + F10` 等
- 控制台会显示每个宏的当前状态
- 按 **Ctrl+C** 退出程序

## 配置选项

每个宏支持以下配置：

| 选项 | 类型 | 示例 | 说明 |
|------|------|------|------|
| `key` | 字符串 | "1" | 要按下的键（1-5, q, w, e, r, t） |
| `interval_ms` | 数字 | 1000 | 基础按键间隔（毫秒） |
| `random_variance_ms` | 数字 | 200 | 随机偏差（±毫秒），0表示无随机性 |
| `toggle_hotkey` | 字符串 | "F9" | 切换此宏的热键（F1-F12） |
| `enabled_by_default` | 布尔值 | false | 启动时是否启用此宏 |

**随机间隔示例：**
- `interval_ms: 1000, random_variance_ms: 200` → 实际间隔：800-1200ms
- `interval_ms: 1500, random_variance_ms: 300` → 实际间隔：1200-1800ms
- `interval_ms: 2000, random_variance_ms: 0` → 实际间隔：固定2000ms

## 支持的按键

### 目标按键
- 数字键: `1`, `2`, `3`, `4`, `5`
- 字母键: `q`, `w`, `e`, `r`, `t`

### 切换热键
- 功能键: `F1` 到 `F12`

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

- `enigo` - 跨平台输入模拟
- `device_query` - 键盘状态检测
- `tokio` - 异步运行时
- `serde` - 序列化
- `serde_yaml` - YAML配置
- `tracing` - 日志记录

## 贡献

欢迎贡献！请随时提交Pull Request。

## 许可证

本项目采用MIT许可证 - 详见LICENSE文件。

## 致谢

- [enigo](https://github.com/enigo-rs/enigo) - 输入模拟库
- [device_query](https://github.com/ostrosco/device_query) - 设备状态库
- POE2社区提供的宏使用指南

## 常见问题

**问：笔记本上F9热键不生效？**
答：大多数笔记本需要按 `Fn + F9` 来触发功能键。

**问：可以使用1-5和Q-T以外的其他键吗？**
答：目前仅支持这些键以符合POE2规则。你可以修改代码添加更多键。

**问：使用这个工具安全吗？**
答：此工具仅模拟键盘输入，不修改游戏内存。但请自行承担风险，并始终遵守POE2的服务条款。

